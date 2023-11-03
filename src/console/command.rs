use std::process;

pub fn clear() -> Result<(), String> {
    match process::Command::new("clear").status() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
