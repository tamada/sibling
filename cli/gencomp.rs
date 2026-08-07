use std::path::Path;

#[cfg(debug_assertions)]
mod generator {
    use clap::{Command, CommandFactory};
    use clap_complete::Shell;
    use std::fs::File;
    use std::path::Path;

    #[cfg(debug_assertions)]
    fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
        let destfile = outdir.join(file);
        std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
        if let Ok(mut dest) = File::create(destfile) {
            clap_complete::generate(s, app, appname, &mut dest);
        }
    }

    pub(super) fn generate(outdir: &Path) {
        use Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
        let appname = "sibling";

        let mut app = crate::cli::CliOpts::command();
        app.set_bin_name(appname);

        generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
        generate_impl(
            Elvish,
            &mut app,
            appname,
            outdir,
            format!("elvish/{appname}"),
        );
        generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
        generate_impl(
            PowerShell,
            &mut app,
            appname,
            outdir,
            format!("powershell/{appname}"),
        );
        generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
    }
}

#[allow(dead_code, unused_variables)]
pub(crate) fn generate(outdir: &Path) {
    #[cfg(debug_assertions)]
    generator::generate(outdir);
}

#[cfg(test)]
mod tests {
    use super::generate;

    /// The completion file of every shell is generated into the given directory.
    /// Note that the release workflow runs it before packaging, hence, the
    /// release ships what this test checks.
    #[test]
    fn test_generate() {
        let outdir = std::env::temp_dir().join(format!("sibling-completions-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&outdir);

        generate(&outdir);

        for path in [
            "bash/sibling",
            "elvish/sibling",
            "fish/sibling",
            "powershell/sibling",
            "zsh/_sibling",
        ] {
            let file = outdir.join(path);
            assert!(file.is_file(), "{path}: not generated");
            let script = std::fs::read_to_string(&file).unwrap();
            // The completion of an option tells that it is built from the
            // current definition, not from a stale one. Note that the name has
            // no leading dashes on some shells, such as fish.
            assert!(script.contains("not-on-dirs"), "{path}: the options are stale");
        }
        std::fs::remove_dir_all(&outdir).unwrap();
    }
}
