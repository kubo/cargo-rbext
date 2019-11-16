use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

mod cmd_build;
mod cmd_install;
mod cmd_new;

#[cfg(not(windows))]
const CDYLIB_PREFIX: &str = "lib";
#[cfg(windows)]
const CDYLIB_PREFIX: &str = "";

#[cfg(all(unix, not(target_os = "macos")))]
const CDYLIB_EXT: &str = "so";
#[cfg(target_os = "macos")]
const CDYLIB_EXT: &str = "dylib";
#[cfg(windows)]
const CDYLIB_EXT: &str = "dll";

#[cfg(not(target_os = "macos"))]
const RUBY_DLEXT: &str = "so";
#[cfg(target_os = "macos")]
const RUBY_DLEXT: &str = "bundle";

fn show_usage() {
    print!("{}", include_str!("data/usage.txt"));
}

fn to_str_opt(s: &Option<String>) -> Option<&str> {
    s.as_ref().map(|n| n.as_ref())
}

fn root_dir() -> PathBuf {
    let mut dir = PathBuf::from(".");
    while dir.exists() {
        if dir.join("Cargo.toml").exists() {
            return dir;
        }
        dir.push("..");
    }
    panic!("Cargo.toml doesn't exists in the current and ancestor directories.");
}

#[derive(Deserialize)]
struct RubyExt {
    extension: Extension,
}

#[derive(Deserialize)]
struct Extension {
    name: String,
}

fn cdylib_name<P: AsRef<Path>>(root_dir: P) -> String {
    let content = fs::read_to_string(root_dir.as_ref().join("Cargo.toml"))
        .expect("failed to read Cargo.toml");
    let value = toml::from_str::<toml::Value>(&content).expect("failed to parse Cargo.toml");
    let name = if let Some(name) = value
        .get("lib")
        .and_then(|lib| lib.get("name"))
        .and_then(|name| name.as_str())
    {
        name
    } else if let Some(name) = value
        .get("package")
        .and_then(|lib| lib.get("name"))
        .and_then(|name| name.as_str())
    {
        name
    } else {
        panic!("Could not find the rust library name");
    };
    format!("{}{}.{}", CDYLIB_PREFIX, name, CDYLIB_EXT)
}

fn extension_name<P: AsRef<Path>>(root_dir: P) -> String {
    let content = fs::read_to_string(root_dir.as_ref().join("RubyExt.toml"))
        .expect("failed to read RubyExt.toml");
    match toml::from_str::<RubyExt>(&content) {
        Ok(ext) => format!("{}.{}", ext.extension.name, RUBY_DLEXT),
        Err(e) => panic!("failed to parse RubyExt.toml: {}", e),
    }
}

fn run_cargo_command(subcommand: &str, args: &[String]) {
    print!("  Run command: cargo {}", subcommand);
    for arg in args.iter() {
        if arg.find(' ').is_some() {
            print!(" \"{}\"", arg);
        } else {
            print!(" {}", arg);
        }
    }
    println!("");
    let cargo_cmd = env::var("CARGO").unwrap_or("cargo".to_string());
    let status = Command::new(cargo_cmd)
        .arg(subcommand)
        .args(args)
        .status()
        .expect(&format!("cargo {} command failed to start", subcommand));
    if !status.success() {
        panic!("cargo {} command failed with {}", subcommand, status);
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let mut first_arg = args.next();
    if to_str_opt(&first_arg) == Some("rbext") {
        first_arg = args.next();
    }

    match to_str_opt(&first_arg) {
        Some("new") => cmd_new::main(args).unwrap(),
        Some("build") => cmd_build::main(args, root_dir()).unwrap(),
        Some("install") => cmd_install::main(args, root_dir()).unwrap(),
        Some(subcmd @ _) => {
            println!("Unknown subcommand: {}", subcmd);
            show_usage();
            process::exit(1);
        }
        None => show_usage(),
    }
}
