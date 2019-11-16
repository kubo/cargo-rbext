# cargo-rbext - Cargo subcommand to use Rust with Ruby

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

## Examples

### Create a ruby extension library written in rust

I have tested on Linux and macOS. It works on Windows if [rosy][] is changed.

```sh
$ cargo rbext new --lib my_ext_name
  Run command: cargo new my_ext_name
     Created binary (application) `my_ext_name` package
  Fix Cargo.toml and src/main.rs
$ cd my_ext_name
$ cargo rbext build
  Run command: cargo build
   Compiling aloxide v0.0.8
   Compiling my_ext_name v0.1.0 (/dirname/my_ext_name)
   Compiling rosy v0.0.9
    Finished dev [unoptimized + debuginfo] target(s) in 2.41s
  Copy ./target/debug/libmy_ext_name.so to ./target/debug/my_ext_name.so
$ ruby -Itarget/debug -rmy_ext_name -e "MyExtName::MyClass.new.hello('Ruby world')"
Hello, Ruby world!
```

### Create a ruby gem package using bundle

```sh
$ bundle gem my_gem_name --ext # create a template of a gem package with '--ext'
Creating gem 'my_gem_name'...
MIT License enabled in config
      create  my_gem_name/Gemfile
      create  my_gem_name/lib/my_gem_name.rb
      create  my_gem_name/lib/my_gem_name/version.rb
      create  my_gem_name/my_gem_name.gemspec
      create  my_gem_name/Rakefile
      create  my_gem_name/README.md
      create  my_gem_name/bin/console
      create  my_gem_name/bin/setup
      create  my_gem_name/.gitignore
      create  my_gem_name/LICENSE.txt
      create  my_gem_name/ext/my_gem_name/extconf.rb
      create  my_gem_name/ext/my_gem_name/my_gem_name.h
      create  my_gem_name/ext/my_gem_name/my_gem_name.c
Initializing git repo in /home/foobar/my_gem_name
$ cd my_gem_name
$ git rm --cached ext/my_gem_name/* # unstage C extension files from the git repository.
rm 'ext/my_gem_name/extconf.rb'
rm 'ext/my_gem_name/my_gem_name.c'
rm 'ext/my_gem_name/my_gem_name.h'
$ rm -rf ext/my_gem_name/ # remove C extension
$ cargo rbext new --lib --name my_gem_name/my_gem_name ext/my_gem_name # add Rust extension
  Run comamnd: cargo new --lib ext/my_gem_name --name my_gem_name
     Created library `my_gem_name` package
  Fix Cargo.toml and src/lib.rs
  Create RubyExt.toml, Rakefile and build.rs
$ git add ext/my_gem_name/* # add Rust extension files to the git repository.
$ sed -i "s/extconf.rb/Rakefile/" my_gem_name.gemspec # or open the file and replace `extconf.rb` with `Rakefile`

... Fix my_gem_name.gemspec until the next step succeeds ...

$ gem build my_gem_name.gemspec
  Successfully built RubyGem
  Name: my_gem_name
  Version: 0.1.0
  File: my_gem_name-0.1.0.gem
$ gem install my_gem_name-0.1.0.gem 
Building native extensions. This could take a while...
Successfully installed my_gem_name-0.1.0
Parsing documentation for my_gem_name-0.1.0
Done installing documentation for my_gem_name after 0 seconds
1 gem installed
$ ruby -rmy_gem_name -e "MyGemName::MyClass.new.hello('Ruby world')"
Hello, Ruby world!
```

Note: The value of the `--name` option in `cargo rbext new --lib` is
`my_gem_name/my_gem_name` because `lib/my_gem_name.rb` requires it.

### Create a rust crate package calling ruby methods

This works only on Linux.

```sh
$ cargo rbext new my_prog_name
  Run command: cargo new my_prog_name
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
