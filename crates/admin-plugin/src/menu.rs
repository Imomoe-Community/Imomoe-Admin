use crate::{config::is_chinese, dispatch, engine, s2sdk as sdk, state::State};
use imomoe_admin_core::{
    command::{Command, lookup},
    target::{Actor, Player},
};
use plugify::Str;
use std::{collections::HashMap, fmt::Write as _, time::Instant};

const PAGE_SIZE: usize = 7;
const SECONDS: i32 = 20;

struct AdminAction {
    name: &'static str,
    label: (&'static str, &'static str),
}

impl AdminAction {
    fn command(&self) -> &'static Command {
        lookup(self.name, false).expect("menu action must be in the command directory")
    }
}

// Only actions that can execute with one selected target and their command defaults.
const ACTIONS: &[AdminAction] = &[
    AdminAction {
        name: "slap",
        label: ("拍击（无伤害）", "Slap (no damage)"),
    },
    AdminAction {
        name: "mute",
        label: ("禁麦", "Mute voice"),
    },
    AdminAction {
        name: "unmute",
        label: ("解除禁麦", "Unmute voice"),
    },
    AdminAction {
        name: "gag",
        label: ("禁言", "Block text chat"),
    },
    AdminAction {
        name: "ungag",
        label: ("解除禁言", "Restore text chat"),
    },
    AdminAction {
        name: "silence",
        label: ("禁麦并禁言", "Block voice and text"),
    },
    AdminAction {
        name: "unsilence",
        label: ("解除禁麦和禁言", "Restore voice and text"),
    },
    AdminAction {
        name: "bring",
        label: ("拉到身边", "Bring player"),
    },
    AdminAction {
        name: "respawn",
        label: ("复活", "Respawn player"),
    },
];

enum View {
    Actions(Vec<&'static AdminAction>),
    Targets(&'static AdminAction, Vec<Option<Player>>),
}

impl View {
    fn len(&self) -> usize {
        match self {
            Self::Actions(actions) => actions.len(),
            Self::Targets(_, players) => players.len(),
        }
    }
}

struct Session {
    view: View,
    page: usize,
    shown: Instant,
}

impl Session {
    fn new(view: View) -> Self {
        Self {
            view,
            page: 0,
            shown: Instant::now(),
        }
    }

    fn has_next(&self) -> bool {
        (self.page + 1) * PAGE_SIZE < self.view.len()
    }

    fn display(&mut self, slot: i32) {
        let zh = is_chinese(engine::phrase_language(Actor::Player(slot)).as_str());
        let text = |zh_text, en_text| if zh { zh_text } else { en_text };
        let label = |action: &AdminAction| text(action.label.0, action.label.1);
        let mut html = String::new();
        match &self.view {
            View::Actions(_) => write!(html, "<b>{}</b>", text("管理菜单", "Admin menu")),
            View::Targets(action, _) => write!(
                html,
                "<b>{}: {}</b>",
                text("选择玩家", "Choose player"),
                label(action)
            ),
        }
        .unwrap();
        write!(
            html,
            "<br>{}<br>",
            text("在聊天中输入数字", "Type a number in chat")
        )
        .unwrap();
        let start = self.page * PAGE_SIZE;
        for row in start..(start + PAGE_SIZE).min(self.view.len()) {
            let number = row - start + 1;
            match &self.view {
                View::Actions(actions) => write!(html, "{number}. {}<br>", label(actions[row])),
                View::Targets(_, players) => {
                    let display = players[row].as_ref().map_or_else(
                        || text("玩家已离开", "Player left").to_owned(),
                        |p| escape(&p.name),
                    );
                    write!(html, "{number}. {display}<br>")
                }
            }
            .unwrap();
        }
        if self.page > 0 {
            write!(html, "8. {}<br>", text("上一页", "Previous")).unwrap();
        }
        if self.has_next() {
            write!(html, "9. {}<br>", text("下一页", "Next")).unwrap();
        }
        let exit = match self.view {
            View::Actions(_) => text("退出", "Exit"),
            View::Targets(..) => text("返回", "Back"),
        };
        write!(html, "0. {exit}").unwrap();
        sdk::PrintCentreHtml(slot, &Str::from_str(&html), SECONDS);
        self.shown = Instant::now();
    }
}

fn escape(text: &str) -> String {
    let mut html = String::new();
    for ch in text.chars().take(40) {
        match ch {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            ch => html.push(ch),
        }
    }
    html
}

#[derive(Default)]
pub struct Menus(HashMap<i32, Session>);

impl Menus {
    pub fn disconnect(&mut self, slot: i32) {
        self.0.remove(&slot);
        for session in self.0.values_mut() {
            if let View::Targets(_, players) = &mut session.view {
                for row in players {
                    if row.as_ref().is_some_and(|p| p.slot == slot) {
                        *row = None;
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for (slot, session) in self.0.drain() {
            if session.shown.elapsed().as_secs() < SECONDS as u64 && engine::online(slot) {
                hide(slot);
            }
        }
    }
}

fn hide(slot: i32) {
    sdk::PrintCentreHtml(slot, &Str::from_str(" "), 1);
}

pub fn open(state: &mut State, actor: Actor) {
    let Actor::Player(slot) = actor else {
        state.reply(actor, "player_only", "");
        return;
    };
    let actions: Vec<_> = ACTIONS
        .iter()
        .filter(|a| dispatch::authorized(actor, a.command()))
        .collect();
    if actions.is_empty() {
        if state.menus.0.remove(&slot).is_some() {
            hide(slot);
        }
        state.reply(actor, "denied", "");
        return;
    }
    let mut session = Session::new(View::Actions(actions));
    session.display(slot);
    state.menus.0.insert(slot, session);
}

pub fn input(state: &mut State, actor: Actor, text: &str) -> bool {
    let Actor::Player(slot) = actor else {
        return false;
    };
    let [digit @ b'0'..=b'9'] = *text.as_bytes() else {
        return false;
    };
    let Some(mut session) = state.menus.0.remove(&slot) else {
        return false;
    };
    if session.shown.elapsed().as_secs() >= SECONDS as u64 {
        return false;
    }
    match digit {
        b'0' => {
            match &session.view {
                View::Actions(_) => hide(slot),
                View::Targets(..) => {
                    // Keep the current session until open redraws or closes it.
                    state.menus.0.insert(slot, session);
                    open(state, actor);
                }
            }
            return true;
        }
        b'8' if session.page > 0 => session.page -= 1,
        b'9' if session.has_next() => session.page += 1,
        b'1'..=b'7' => {
            let row = session.page * PAGE_SIZE + usize::from(digit - b'1');
            if row >= session.view.len() {
                state.menus.0.insert(slot, session);
                return false;
            }
            match &session.view {
                View::Actions(actions) => {
                    let action = actions[row];
                    let cmd = action.command();
                    if !dispatch::authorized(actor, cmd) {
                        hide(slot);
                        state.reply(actor, "denied", "");
                        return true;
                    }
                    let players: Vec<_> = engine::players()
                        .into_iter()
                        .filter(|p| {
                            p.steam_id.is_some()
                                || (p.bot && dispatch::online_punishments(cmd).is_empty())
                        })
                        .filter(|p| {
                            p.slot != slot || state.config.admin.allow_self.contains(cmd.name)
                        })
                        .map(Some)
                        .collect();
                    if players.is_empty() {
                        state.reply(actor, "not_found", "");
                    } else {
                        session = Session::new(View::Targets(action, players));
                    }
                }
                View::Targets(action, players) => {
                    hide(slot);
                    if let Some(target) = &players[row]
                        && engine::players().iter().any(|p| {
                            p.slot == target.slot
                                && p.user_id == target.user_id
                                && p.steam_id == target.steam_id
                                && p.bot == target.bot
                        })
                    {
                        let selector = target.steam_id.map_or_else(
                            || format!("#userid:{}", target.user_id),
                            |id| id.to_string(),
                        );
                        dispatch::run(state, actor, action.command(), &[selector]);
                    } else {
                        state.reply(actor, "not_found", "");
                    }
                    return true;
                }
            }
        }
        _ => {
            state.menus.0.insert(slot, session);
            return false;
        }
    }
    session.display(slot);
    state.menus.0.insert(slot, session);
    true
}
