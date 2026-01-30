pub const CODE_OK: i32 = 0;

pub mod category {
    pub const SUCCESS: i32 = 0;
    pub const BUSINESS_ERROR: i32 = 1;
    pub const PROCESSING: i32 = 2;
    pub const PARTIAL_SUCCESS: i32 = 3;
    pub const UNKNOWN_ERROR: i32 = 9;
}

pub mod module {
    pub const AUTH: i32 = 1;
    pub const USER: i32 = 2;
    pub const ORG: i32 = 3;
    pub const PROJECT: i32 = 4;
    pub const SPACE: i32 = 5;
    pub const WORKFLOW: i32 = 6;
    pub const NODE: i32 = 7;
    pub const SUBTASK: i32 = 8;
    pub const LINK: i32 = 9;
    pub const DOC: i32 = 10;
    pub const COMMENT: i32 = 11;
    pub const NOTIFICATION: i32 = 12;
    pub const COMMON: i32 = 99;
}

/// Build a business code using the CMMRR scheme.
/// C: category, MM: module, RR: reason.
pub const fn make_code(category: i32, module: i32, reason: i32) -> i32 {
    (category * 10000) + (module * 100) + reason
}
