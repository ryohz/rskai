pub struct Command {
    pub name: String,
    // variable to store function that will be called when the paired name is used on the prompt
    // when it's called, arguments of the command is passwd via a string argument.
    pub func: Box<dyn Fn(String) -> super::types::IsError>,
}

impl Command {
    pub fn new(name: &str, func: Box<dyn Fn(String) -> super::types::IsError>) -> Self {
        Command {
            name: name.to_string(),
            func,
        }
    }
}

pub fn builtins() -> Vec<Command> {
    vec![Command::new("clear", Box::new(clear_console))]
}

pub fn clear_console(arg: String) -> super::types::IsError {
    let result = super::console::command::clear();
    if result.is_err() {
        super::console::output::errorln!("failed to clear console: {}", result.unwrap_err());
        super::types::IsError::from(true)
    } else {
        super::types::IsError::from(false)
    }
}
