use rust_embed::Embed;

use sibling::{Error, Result};

#[derive(Embed)]
#[folder = "assets/init"]
struct Assets;

/// The shells which the initialize script is available for.
/// The `bash` script works on zsh, too.
const SUPPORTED_SHELLS: &str = "bash, zsh, fish, powershell, elvish";

pub(crate) fn generate_init_script(shell_name: &str) -> Result<String> {
    let script_file = match shell_name.to_lowercase().as_str() {
        "bash" | "zsh" => "init.bash",
        "fish" => "init.fish",
        "powershell" | "pwsh" => "init.ps1",
        "elvish" | "elv" => "init.elv",
        _ => {
            return Err(Error::Fatal(format!(
                "{shell_name}: Unsupported shell (supported: {SUPPORTED_SHELLS})"
            )));
        }
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
    use super::generate_init_script;

    /// Every supported shell gets the script which defines the utility commands.
    #[test]
    fn test_generate_init_script() {
        for shell in ["bash", "zsh", "Bash", "fish", "powershell", "pwsh", "elvish", "elv"] {
            let script = generate_init_script(shell)
                .unwrap_or_else(|e| panic!("{shell}: failed to generate the script: {e}"));
            for command in ["cdnext", "cdprev", "cdfirst", "cdlast", "cdrand"] {
                assert!(script.contains(command), "{shell}: {command} is not defined");
            }
        }
    }

    /// The bash script is shared with zsh, while the others are their own.
    #[test]
    fn test_generate_init_script_of_each_shell() {
        let bash = generate_init_script("bash").unwrap();
        assert_eq!(generate_init_script("zsh").unwrap(), bash);
        assert_eq!(
            generate_init_script("powershell").unwrap(),
            generate_init_script("pwsh").unwrap()
        );
        assert_eq!(
            generate_init_script("elvish").unwrap(),
            generate_init_script("elv").unwrap()
        );
        for shell in ["fish", "powershell", "elvish"] {
            assert_ne!(generate_init_script(shell).unwrap(), bash, "{shell}");
        }
    }

    #[test]
    fn test_generate_init_script_with_unsupported_shell() {
        let e = generate_init_script("csh").expect_err("csh should be unsupported");
        assert!(e.to_string().contains("csh: Unsupported shell"));
    }
}