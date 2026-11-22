#[derive(Clone, Copy)]
pub struct Command {
    pub name: &'static str,
    pub chat: &'static str,
    pub permissions: &'static [&'static str],
    pub usage: &'static str,
    pub multi: bool,
}

macro_rules! command {
    ($name:literal, $chat:literal, [$($perm:literal),*], $usage:literal $(, $multi:literal)?) => {
        Command { name: $name, chat: $chat, permissions: &[$(concat!("imomoe.admin.", $perm)),*], usage: $usage, multi: false $(|| $multi)? }
    };
}

pub const COMMANDS: &[Command] = &[
    command!("kick", "kick", ["kick"], "<target> [reason]"),
    command!("ban", "ban", ["ban"], "<target> [duration] [reason]"),
    command!(
        "addban",
        "addban",
        ["ban"],
        "<SteamID64> [duration] [reason]"
    ),
    command!("unban", "unban", ["unban"], "<SteamID64>"),
    command!("banlist", "banlist", ["ban"], "[page]"),
    command!(
        "mute",
        "mute",
        ["mute"],
        "<target> [duration] [reason]",
        true
    ),
    command!("unmute", "unmute", ["mute"], "<target>", true),
    command!("gag", "gag", ["gag"], "<target> [duration] [reason]", true),
    command!("ungag", "ungag", ["gag"], "<target>", true),
    command!(
        "silence",
        "silence",
        ["mute", "gag"],
        "<target> [duration] [reason]",
        true
    ),
    command!("unsilence", "unsilence", ["mute", "gag"], "<target>", true),
    command!("comms", "comms", [], ""),
    command!("slay", "slay", ["slay"], "<target>", true),
    command!("slap", "slap", ["slay"], "<target> [damage]", true),
    command!("team", "team", ["team"], "<target> <t|ct|spec>", true),
    command!("swap", "swap", ["team"], "<target>", true),
    command!("respawn", "respawn", ["respawn"], "<target>", true),
    command!(
        "tp",
        "tp",
        ["teleport"],
        "<target> <destination player>",
        true
    ),
    command!("bring", "bring", ["teleport"], "<target>", true),
    command!("goto", "goto", ["teleport"], "<destination player>"),
    command!("freeze", "freeze", ["cheat"], "<target>", true),
    command!("unfreeze", "unfreeze", ["cheat"], "<target>", true),
    command!("noclip", "noclip", ["cheat"], "<target>"),
    command!("who", "who", ["who"], ""),
    command!("hide", "hide", ["hide"], ""),
    command!("adminchat", "a", ["chat"], "<message>"),
    command!("restart", "restart", ["match"], "[delay seconds]"),
    command!("pause", "pause", ["match"], ""),
    command!("unpause", "unpause", ["match"], ""),
    command!("cvar", "cvar", ["cvar"], "<name> [value]"),
    command!("reload", "iareload", ["reload"], ""),
    command!("admins", "admins", [], ""),
    command!("report", "report", [], "<message>"),
    command!("rules", "rules", [], ""),
    command!("help", "help", [], ""),
];

pub fn lookup(name: &str, chat: bool) -> Option<&'static Command> {
    COMMANDS
        .iter()
        .find(|cmd| (if chat { cmd.chat } else { cmd.name }).eq_ignore_ascii_case(name))
}

/// Parse the inner chat string without letting a quote, semicolon or newline
/// turn an argument into a second engine command.
pub fn words(input: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut value = String::new();
    let mut quoted = false;
    let mut started = false;
    for ch in input.chars() {
        match ch {
            '"' => {
                quoted = !quoted;
                started = true;
            }
            ch if ch.is_whitespace() && !quoted => {
                if started {
                    out.push(std::mem::take(&mut value));
                    started = false;
                }
            }
            ch => {
                value.push(ch);
                started = true;
            }
        }
    }
    if quoted {
        return None;
    }
    if started {
        out.push(value);
    }
    Some(out)
}

pub fn private_input(say_team: bool, text: &str) -> Option<(&'static str, &str)> {
    text.trim_start().strip_prefix('@').map(|message| {
        (
            if say_team { "adminchat" } else { "report" },
            message.trim(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_private_messages_and_quoted_targets_as_data() {
        assert_eq!(
            private_input(false, " @name;hello"),
            Some(("report", "name;hello"))
        );
        assert_eq!(
            private_input(true, " @team only"),
            Some(("adminchat", "team only"))
        );
        assert!(private_input(false, "!report test").is_none());
        assert_eq!(
            words("slap \"Player Name\" 10").unwrap(),
            ["slap", "Player Name", "10"]
        );
        assert_eq!(words("cvar mock \"a;b\"").unwrap(), ["cvar", "mock", "a;b"]);
        assert!(words("ban \"incomplete").is_none());
        assert_eq!(lookup("a", true).unwrap().name, "adminchat");
        assert_eq!(
            lookup("silence", false).unwrap().permissions,
            ["imomoe.admin.mute", "imomoe.admin.gag"]
        );
    }
}
