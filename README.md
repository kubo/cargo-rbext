# cargo-rbext - Cargo subcommand to use Rust from Ruby and vice varsa

## Status

It is unstable. I'll focus on other projects in 2019. If it interests you, feel free to fork and develop.

This supports [rosy][] only now.

## Installation

```sh
$ git clone https://github.com/kubo/cargo-rbext.git
$ cd cargo-rbext
$ cargo install --path .
```
## Usage

* [cargo rbext new](src/data/usage-new.txt)
* [cargo rbext build](src/data/usage-build.txt) (Use this only for libraries)
* [cargo rbext install](src/data/usage-install.txt) (Use this only for libraries)

## Example

### Create a Ruby extension library written in Rust

I have tested on Linux and macOS. It works on Windows if [rosy][] is changed.

```sh
$ cargo rbext new --lib my_ext_name
  Run comamnd: cargo new my_ext_name
     Created binary (application) `my_ext_name` package
  Fix Cargo.toml and src/main.rs
$ cd my_ext_name
$ cargo rbext build
  Run comamnd: cargo build
   Compiling aloxide v0.0.8
   Compiling my_ext_name v0.1.0 (/dirname/my_ext_name)
   Compiling rosy v0.0.9
    Finished dev [unoptimized + debuginfo] target(s) in 2.41s
  Copy ./target/debug/libmy_ext_name.so to ./target/debug/my_ext_name.so
$ ruby -Itarget/debug -rmy_ext_name -e "MyExtName.new.hello('Ruby world')"
Hello, Ruby world!
```

### Crate a Rust crate package calling Ruby methods

This works only on Linux.

```sh
$ cargo rbext new my_prog_name
  Run comamnd: cargo new my_prog_name
     Created binary (application) `my_prog_name` package
  Fix Cargo.toml and src/main.rs
$ cd my_prog_name
$ cargo build
    Updating crates.io index
   Compiling aloxide v0.0.8
   Compiling rosy v0.0.9
   Compiling my_prog_name v0.1.0 (/dirname/my_prog_name)
    Finished dev [unoptimized + debuginfo] target(s) in 3.38s
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/my_prog_name`
Hello, Ruby world!
```

## License

This program itself is under BSD 2-Clause "Simplified" License.

The license doesn't cover the files copied by `cargo rbext new`.

[rosy]: https://github.com/oceanpkg/rosy
