use rust_embed::Embed;

use sibling::{Result, SiblingError};

#[derive(Embed)]
#[folder = "../assets/init"]
struct Assets;

pub(crate) fn generate_init_script(shell_name: String) -> Result<String> {
    let script_file = match shell_name.to_lowercase().as_str() {
        "bash" => "init.bash",
        "zsh" => "init.bash",
        _ => {
            return Err(SiblingError::Fatal(format!(
                "{shell_name}: Unsupported shell"
            )))
        }
    };
    match Assets::get(script_file) {
        Some(file) => match std::str::from_utf8(file.data.as_ref()) {
            Ok(script) => Ok(script.to_string()),
            Err(_) => Err(SiblingError::Fatal(format!(
                "{script_file}: Invalid script"
            ))),
        },
        None => Err(SiblingError::NotFound(script_file.into())),
    }
}
