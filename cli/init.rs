use rust_embed::Embed;

use sibling::{Error, Result};

#[derive(Embed)]
#[folder = "assets/init"]
struct Assets;

pub(crate) fn generate_init_script(shell_name: &str) -> Result<String> {
    let script_file = match shell_name.to_lowercase().as_str() {
        "bash" | "zsh" => "init.bash",
        _ => return Err(Error::Fatal(format!("{shell_name}: Unsupported shell"))),
    };
    match Assets::get(script_file) {
        Some(file) => match std::str::from_utf8(file.data.as_ref()) {
            Ok(script) => Ok(script.to_string()),
            Err(_) => Err(Error::Fatal(format!("{script_file}: Invalid script"))),
        },
        None => Err(Error::NotFound(script_file.into())),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_init_script_with_bash() {
        let script = super::generate_init_script("bash");
        assert!(script.is_ok());
    }

    #[test]
    fn test_generate_init_script_with_unsupported_shell() {
        let script = super::generate_init_script("fish");
        assert!(script.is_err());
    }
}