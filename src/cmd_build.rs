use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::PathBuf;

use super::{cdylib_name, extension_name, run_cargo_command};

fn basename(name: &str) -> &str {
    if let Some(idx) = name.rfind('/') {
        name.split_at(idx + 1).1
    } else {
        name
    }
}

pub fn main<T: Iterator<Item = String>>(args: T, root_dir: PathBuf) -> io::Result<()> {
    let args: Vec<String> = args.collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print!("{}", include_str!("data/usage-build.txt"));
        return Ok(());
    }

    run_cargo_command("build", &args);

    let out_dir = if args.contains(&"--release".to_string()) {
        "target/release"
    } else {
        "target/debug"
    };
    let out_dir = root_dir.join(out_dir);
    let src = out_dir.join(cdylib_name(&root_dir));
    let dest = out_dir.join(basename(&extension_name(&root_dir)));
    println!("  Copy {} to {}", src.display(), dest.display());
    fs::copy(src, dest)?;
    Ok(())
}
