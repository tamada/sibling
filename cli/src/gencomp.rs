#[cfg(debug_assertions)]
use clap::{Command, CommandFactory};
use clap_complete::Shell;
use std::fs::File;
use std::path::{Path, PathBuf};

fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();
    
    clap_complete::generate(s, app, appname, &mut dest);
}

pub fn generate(outdir: PathBuf) {
    let appname = "sibling";

    let mut app = crate::cli::CliOpts::command();
    app.set_bin_name(appname);

    generate_impl(Shell::Bash, &mut app, appname, &outdir, format!("bash/{}", appname));
    generate_impl(Shell::Elvish, &mut app, appname, &outdir, format!("elvish/{}", appname));
    generate_impl(Shell::Fish, &mut app, appname, &outdir, format!("fish/{}", appname));
    generate_impl(Shell::PowerShell, &mut app, appname, &outdir, format!("powershell/{}", appname));
    generate_impl(Shell::Zsh, &mut app, appname, &outdir, format!("zsh/_{}", appname));
}
