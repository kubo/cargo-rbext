#[cfg(windows)]
mod osdep {
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    fn ruby_config() -> HashMap<String, String> {
        let output = Command::new("ruby")
            .arg("-e")
            .arg("RbConfig::CONFIG.each { |k, v| puts(\"#{k}=#{v}\") }")
            .output()
            .expect("failed to execute ruby");
        if !output.status.success() {
            panic!("Ruby process exited with status {}", output.status);
        }
        let mut cfg = HashMap::new();
        for line in String::from_utf8_lossy(&output.stdout).split('\n') {
            if let Some(pos) = line.find('=') {
                cfg.insert(
                    line[0..pos].to_string(),
                    line[(pos + 1)..].trim_end().to_string(),
                );
            }
        }
        cfg
    }

    pub fn print_cdylib_link_arg() {
        let cfg = ruby_config();
        let libdir = cfg
            .get("libdir")
            .expect("Could not get 'libdir' in RbConfig::CONFIG")
            .replace('/', "\\");
        let libruby = cfg
            .get("LIBRUBY")
            .expect("Could not get 'LIBRUBY' in RbConfig::CONFIG");
        let src_file = Path::new(&libdir).join(libruby);

        let target = env::var("TARGET").expect("Could not get 'TARGET'");
        if target.ends_with("gnu") {
            // gnu toolchain
            println!("cargo:rustc-cdylib-link-arg={}", src_file.display());
        } else {
            // msvc toolchain cannot use lib files ending with '.a'.
            // copy ruby's lib file as ruby.lib and use it.
            let out_dir = env::var("OUT_DIR").expect("Could not get 'OUT_DIR'");
            let dest_file = Path::new(&out_dir).join("ruby.lib");
            fs::copy(&src_file, &dest_file).expect(&format!(
                "Failed to copy {} to {}",
                src_file.display(),
                dest_file.display()
            ));
            println!("cargo:rustc-cdylib-link-arg=/LIBPATH:{}", out_dir);
            println!("cargo:rustc-cdylib-link-arg=ruby.lib");
        }
    }
}

#[cfg(target_os = "macos")]
mod osdep {
    pub fn print_cdylib_link_arg() {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup");
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
mod osdep {
    pub fn print_cdylib_link_arg() {}
}

fn main() {
    osdep::print_cdylib_link_arg();
}
