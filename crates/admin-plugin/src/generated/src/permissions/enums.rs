// Generated from permissions.pplugin

#[allow(unused_imports)]
use plugify::vector_enum_traits;

/// Result codes returned by the API. Values after ErrorsStart are errors.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Operation completed successfully.
    Success = 0,
    /// Permission is allowed.
    Allow = 1,
    /// Permission is explicitly denied.
    Disallow = 2,
    /// Permission is not defined.
    PermNotFound = 3,
    /// User has the group temporarily.
    TemporalGroup = 4,
    /// User has the group permanently.
    PermanentGroup = 5,
    /// User doesn't have the group.
    GroupNotDefined = 6,
    /// Marker: every value greater than this is an error.
    ErrorsStart = 30,
    /// Permission already exists with the same state, or conflicts with an existing wildcard.
    PermAlreadyGranted = 31,
    /// Cookie (user) or option (group) is not set.
    CookieNotFound = 32,
    /// Group doesn't exist.
    GroupNotFound = 33,
    /// Child group doesn't exist.
    ChildGroupNotFound = 34,
    /// Parent group doesn't exist, or the group has no parent.
    ParentGroupNotFound = 35,
    /// Group already exists, or the user already has it.
    GroupAlreadyExist = 36,
    /// Setting the parent would create a cycle in the group hierarchy.
    GroupHierarchyCycle = 37,
    /// Actor user is not loaded.
    ActorUserNotFound = 38,
    /// Target user is not loaded.
    TargetUserNotFound = 39,
    /// User is already loaded.
    UserAlreadyExist = 40,
    /// Callback is null.
    CallbackInvalid = 41,
    /// Callback is already registered.
    CallbackAlreadyExist = 42,
    /// Callback is not registered.
    CallbackNotFound = 43,
    /// Storage error.
    StorageError = 44,
    /// A storage callback rejected the change (storage not ready or failed). The change wasn't applied.
    DBNotReady = 45,
    /// Permission line is malformed (empty, empty segment, or '*' not as the last segment).
    InvalidPermission = 46,
    /// Group name is empty.
    InvalidGroupName = 47,
    /// Cookie or option name is empty.
    InvalidCookieName = 48,
    /// Permission already exists with the opposite state. Use Set* to change it.
    PermConflict = 49,
}
vector_enum_traits!(Status, i32);

/// Ownership type for RAII wrappers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    Borrowed,
    Owned,
}
