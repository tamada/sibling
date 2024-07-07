use rust_embed::Embed;

use crate::cli::{Result, SiblingError};

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

pub(crate) fn generate_init_script(shell_name: String) -> Result<String> {
    let script_file = match shell_name.to_lowercase().as_str() {
        "bash" => "init.bash",
        "zsh" => "init.bash",
        _ => return Err(SiblingError::Fatal(format!("{}: Unsupported shell", shell_name))),
    };
    match Assets::get(script_file) {
        Some(file) => match std::str::from_utf8(file.data.as_ref()) {
            Ok(script) => Ok(script.to_string()),
            Err(_) => Err(SiblingError::Fatal(format!("{}: Invalid script", script_file))),
        },
        None => Err(SiblingError::NotFound(script_file.into())),
    }
}
