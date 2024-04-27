use std::{env, hash::Hash};
use std::collections::HashMap;
use std::process::exit;

use self::structures::*;

mod structures;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamAction {
    Generate,
}

#[derive(Debug, Clone, Copy)]
pub enum SubAction{
    Alphabetic,
    NoSymbols,
}
// too long to not alias it
type ActionMap = HashMap<ParamAction, Vec<SubAction>>;

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn find_action(elem: &String) -> Option<ParamAction> {
    if elem.starts_with("--") {
        for param in ACTIONS {
            if elem.eq(&param.long) {
                return Some(param.action);
            }
        }
    } else if elem.starts_with("-") {
        for param in ACTIONS {
            if elem.eq(&param.short) {
                return Some(param.action);
            }
        }
    }
    None
}

fn find_subaction(elem: &String) -> Option<SubAction> {
    if elem.starts_with("--") {
        for param in SUBACTIONS {
            if elem.eq(&param.long) {
                return Some(param.subaction);
            }
        }
    } else if elem.starts_with("-") {
        for param in SUBACTIONS {
            if elem.eq(&param.short) {
                return Some(param.subaction);
            }
        }
    }
    None
}

pub fn set_actions() -> ActionMap {
    let args = get_args();
    let mut res: ActionMap = HashMap::new();
    let mut subactions: Vec<SubAction> = Vec::new();

    let mut args_it = args.iter();
    args_it.next();

    for elem in args_it {
        if elem.starts_with("--") || elem.starts_with("-") {
            if let Some(action) = find_action(elem) {
                res.insert(action, Vec::new());
            } else {
                match find_subaction(elem) {
                    Some(subaction) => subactions.push(subaction),
                    None => {
                        eprintln!("\"{}\" is not a valid argument!", elem);
                        exit(1);
                    }
                }
            }
        }
    }
    res
} 