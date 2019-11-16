use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use super::{cdylib_name, run_cargo_command};

fn archdir() -> PathBuf {
    let archdir = if let Ok(dir) = env::var("RUBYARCHDIR") {
        dir
    } else {
        let output = Command::new("ruby")
            .arg("-e")
            .arg("print RbConfig::CONFIG['sitearchdir']")
            .output()
            .expect("failed to execute ruby");
        if !output.status.success() {
            panic!("Ruby process exited with status {}", output.status);
        }
        String::from_utf8(output.stdout).unwrap()
    };
    let archdir = PathBuf::from(archdir);
    if !archdir.is_dir() {
        panic!("The ruby archdir doesn't exist: {}", archdir.display());
    }
    archdir
}

pub fn main<T: Iterator<Item = String>>(args: T, root_dir: PathBuf) -> io::Result<()> {
    let mut args: Vec<String> = args.collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print!("{}", include_str!("data/usage-install.txt"));
        return Ok(());
    }

    let archdir = archdir();
    let extension_name = super::extension_name(&root_dir);

    args.push("--release".to_string());
    run_cargo_command("build", &args);

    let src = root_dir
        .join("target")
        .join("release")
        .join(cdylib_name(&root_dir));
    let dest = archdir.join(extension_name);
    println!("  Copy {} to {}", src.display(), dest.display());
    fs::create_dir_all(dest.parent().unwrap())?;
    fs::copy(src, dest)?;
    Ok(())
}
