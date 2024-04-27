use jbrspass::console_parser::{set_actions, ParamAction};
fn main() {
    let actions = set_actions();

    for action in actions {
        println!("{:?}", action);
    }
}
