use std::env;
use self::structures::*;

mod structures;

#[derive(Debug, Clone, Copy)]
pub enum ParamAction {
    Generate,
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

pub fn set_actions() -> Vec<ParamAction>  {
    let args = get_args();
    let mut res: Vec<ParamAction> = Vec::new();

    let mut args_it = args.iter();
    args_it.next();

    for elem in args_it {
        if elem.starts_with("--") {
            for param in ACTIONS {
                if elem.eq(&param.long) {
                    res.push(param.action);
                    break;
                }
            }
        } else if elem.starts_with("-") {
            for param in ACTIONS {
                if param.short.ne("NONE") && elem.eq(&param.short) {
                    res.push(param.action);
                    break;
                }
            }
        }
    }
    res
} 