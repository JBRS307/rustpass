use super::ParamAction;


// I KNOW WHAT I AM DOING
// Params are not meant to be created outside
// const ACTIONS
pub struct Param {
    pub action: ParamAction, 
    pub long: &'static str,
    pub short: &'static str,
}

// subargs to modify actions
// same case as with actions, that's
// why static str are usable
pub struct Subarg {
    pub action: ParamAction,
    pub long: &'static str,
    pub short: &'static str,
}

pub const ACTIONS: &[Param] = &[
    Param {
        action: ParamAction::Generate,
        long: "--generate",
        short: "-g",
    },
];

pub const SUBARGS: &[Subarg] = &[
    Subarg {
        action: ParamAction::Generate,
        long: "--letters-only",
        short: "NONE"
    },
    Subarg {
        action: ParamAction::Generate,
        long: "--no-symbols",
        short: "-ns",
    },
];