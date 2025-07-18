#[cfg(debug_assertions)]
mod generator {
    use clap::{Command, CommandFactory};
    use clap_complete::Shell;
    use std::fs::File;
    use std::path::{Path, PathBuf};

    #[cfg(debug_assertions)]
    fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
        let destfile = outdir.join(file);
        std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
        if let Ok(mut dest) = File::create(destfile) {
            clap_complete::generate(s, app, appname, &mut dest);
        }
    }

    pub(super) fn generate(outdir: PathBuf) {
        let appname = "sibling";

        let mut app = crate::cli::CliOpts::command();
        app.set_bin_name(appname);

        generate_impl(
            Shell::Bash,
            &mut app,
            appname,
            &outdir,
            format!("bash/{}", appname),
        );
        generate_impl(
            Shell::Elvish,
            &mut app,
            appname,
            &outdir,
            format!("elvish/{}", appname),
        );
        generate_impl(
            Shell::Fish,
            &mut app,
            appname,
            &outdir,
            format!("fish/{}", appname),
        );
        generate_impl(
            Shell::PowerShell,
            &mut app,
            appname,
            &outdir,
            format!("powershell/{}", appname),
        );
        generate_impl(
            Shell::Zsh,
            &mut app,
            appname,
            &outdir,
            format!("zsh/_{}", appname),
        );
    }
}

#[allow(dead_code, unused_variables)]
pub(crate) fn generate(outdir: std::path::PathBuf) {
    #[cfg(debug_assertions)]
    generator::generate(outdir);
}
