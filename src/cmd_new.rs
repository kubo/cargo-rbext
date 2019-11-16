use case::CaseExt;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;
use std::path::Path;
use std::process::{self, Command};

fn basename(name: &str) -> &str {
    if let Some(idx) = name.rfind('/') {
        name.split_at(idx + 1).1
    } else {
        name
    }
}

enum Opt {
    NoArg,
    OneArg,
}

static OPTS: [(&str, Opt); 17] = [
    ("-q", Opt::NoArg),
    ("--quiet", Opt::NoArg),
    ("--registry", Opt::OneArg),
    ("--vcs", Opt::OneArg),
    ("--bin", Opt::NoArg),
    ("--lib", Opt::NoArg),
    ("--edition", Opt::OneArg),
    ("--name", Opt::OneArg),
    ("-v", Opt::NoArg),
    ("--verbose", Opt::NoArg),
    ("--color", Opt::OneArg),
    ("--frozen", Opt::NoArg),
    ("--locked", Opt::NoArg),
    ("--offline", Opt::NoArg),
    ("-Z", Opt::OneArg),
    ("-h", Opt::OneArg),
    ("--help", Opt::OneArg),
];

struct Options {
    path: String,
    name: Option<String>,
    lib: bool,
    help: bool,
    cargo_args: Vec<String>,
}

impl Options {
    fn new<T: Iterator<Item = String>>(mut args: T) -> Option<Options> {
        let mut path = None;
        let mut name = None;
        let mut lib = false;
        let mut help = false;
        let mut cargo_args = vec![];
        while let Some(arg) = args.next() {
            if arg.starts_with("-") {
                if let Some(ref opt) =
                    OPTS.iter()
                        .find_map(|opt| if opt.0 == &arg { Some(&opt.1) } else { None })
                {
                    let optarg = match opt {
                        Opt::OneArg => Some(args.next()?),
                        Opt::NoArg => None,
                    };
                    let mut addarg = true;
                    match arg.as_ref() {
                        "--lib" => lib = true,
                        "-h" | "--help" => help = true,
                        "--name" => {
                            name = optarg.clone();
                            addarg = false;
                        }
                        _ => (),
                    }
                    if addarg {
                        cargo_args.push(arg);
                        if let Some(optarg) = optarg {
                            cargo_args.push(optarg);
                        }
                    }
                } else {
                    return None;
                }
            } else {
                if path.is_some() {
                    return None;
                }
                path = Some(arg.clone());
                cargo_args.push(arg);
            }
        }
        if let Some(ref n) = name {
            cargo_args.push("--name".to_string());
            cargo_args.push(if lib {
                basename(&n).to_string()
            } else {
                n.to_string()
            })
        }
        Some(Options {
            path: path?,
            name: name,
            lib: lib,
            help: help,
            cargo_args: cargo_args,
        })
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn name(&self) -> &str {
        basename(self.name.as_ref().unwrap_or(&self.path))
    }

    fn ext_name(&self) -> &str {
        self.name
            .as_ref()
            .map(|n| n.as_str())
            .unwrap_or(basename(&self.path))
    }

    fn cargo_args(&self) -> &[String] {
        &self.cargo_args
    }

    fn is_lib(&self) -> bool {
        self.lib
    }
}

fn print_usage_then_exit(exit_code: i32) {
    print!("{}", include_str!("data/usage-new.txt"));
    process::exit(exit_code)
}

fn append_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_file(path: &Path, content: &str, key_values: &[(&str, &str)]) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut content = content.to_string();
    for kv in key_values {
        content = content.replace(kv.0, kv.1);
    }
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn insert_toml_line(path: &Path, table_name: &str, key_value: &str) -> io::Result<()> {
    let mut lines = vec![];
    let mut state = 0;
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        match state {
            0 if &line == table_name => state = 1,
            1 if &line == "" || line.starts_with("[") => {
                lines.push(key_value.to_string());
                state = 2;
            }
            _ => (),
        }
        lines.push(line);
    }
    let mut writer = BufWriter::new(File::create(path)?);
    for line in lines {
        writeln!(writer, "{}", line)?;
    }
    Ok(())
}

pub fn main<T: Iterator<Item = String>>(args: T) -> io::Result<()> {
    let opts = Options::new(args);
    if opts.is_none() {
        print_usage_then_exit(1);
    }
    let opts = opts.unwrap();
    if opts.help {
        print_usage_then_exit(0);
    }
    let dir_path = Path::new(opts.path());

    print!("  Run comamnd: cargo new");
    for arg in opts.cargo_args() {
        if arg.find(' ').is_some() {
            print!(" \"{}\"", arg);
        } else {
            print!(" {}", arg);
        }
    }
    println!("");
    let cargo_cmd = env::var("CARGO").unwrap_or("cargo".to_string());
    let status = Command::new(cargo_cmd)
        .arg("new")
        .args(opts.cargo_args())
        .status()
        .expect("cargo new command failed to start");
    if !status.success() {
        panic!("cargo new command failed with {}", status);
    }

    let base_name = opts.name();
    let ext_name = opts.ext_name();
    let class_name = base_name.to_camel();
    let key_values = [
        ("@EXT_BASENAME@", base_name),
        ("@EXT_CLASSNAME@", &class_name),
        ("@EXT_FULLNAME@", ext_name),
    ];

    if opts.is_lib() {
        println!("  Fix Cargo.toml and src/lib.rs");
        let cargo_toml = dir_path.join("Cargo.toml");
        insert_toml_line(&cargo_toml, "[package]", "build = \"build.rs\"")?;
        append_file(&cargo_toml, include_str!("data/rosy-lib/Cargo.toml.append"))?;
        create_file(
            &dir_path.join("src").join("lib.rs"),
            include_str!("data/rosy-lib/src/lib.rs"),
            &key_values,
        )?;

        println!("  Create RubyExt.toml, Rakefile and build.rs");
        create_file(
            &dir_path.join("RubyExt.toml"),
            include_str!("data/rosy-lib/RubyExt.toml"),
            &key_values,
        )?;
        create_file(
            &dir_path.join("Rakefile"),
            include_str!("data/rosy-lib/Rakefile"),
            &key_values,
        )?;
        create_file(
            &dir_path.join("build.rs"),
            include_str!("data/rosy-lib/build.rs"),
            &key_values,
        )?;
    } else {
        println!("  Fix Cargo.toml and src/main.rs");
        append_file(
            &dir_path.join("Cargo.toml"),
            include_str!("data/rosy-bin/Cargo.toml.append"),
        )?;
        create_file(
            &dir_path.join("src").join("main.rs"),
            include_str!("data/rosy-bin/src/main.rs"),
            &key_values,
        )?;
    }
    Ok(())
}
