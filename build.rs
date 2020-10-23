extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let mut libnice_include_dirs: Vec<String> = Vec::new();

    match pkg_config::Config::new()
        .atleast_version("0.1.0")
        .probe("nice") {
            Ok(libnice) => {
                println!("Found libnice via pkg-config (Version: {})", libnice.version);
                for path in libnice.include_paths {
                    libnice_include_dirs.push(String::from(path.to_str().expect("invalid path")));
                }
            },
            Err(error) => {
                println!("Pkg-config hasn't found libnice: {}.", error);

                let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libnice");
                if !output_path.join("lib").join("nice.lib").exists() && !output_path.join("lib").join("libnice.a").exists() {
                    build_meson(&std::env::current_dir().unwrap().join("libnice"), &output_path, false);
                }

                println!("cargo:rustc-link-lib=dylib=nice");
                println!("cargo:rustc-link-lib=dylib=bcrypt");
                println!("cargo:rustc-link-lib=dylib=Iphlpapi");
                println!("cargo:rustc-link-search=native={}", output_path.join("lib").to_string_lossy());
                println!("cargo:rustc-link-search=native={}", output_path.join("bin").to_string_lossy()); /* to set the PATH environment variable later */

                libnice_include_dirs.push(output_path.join("include").to_string_lossy().into());
                libnice_include_dirs.push(output_path.join("include").join("glib-2.0").to_string_lossy().into());
                libnice_include_dirs.push(output_path.join("lib").join("glib-2.0").join("include").to_string_lossy().into());
            }
    }

    /*
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    if !out_path.join("bindings.rs").exists() {
        let bindings = bindgen::Builder::default()
            .header_contents(
                "wrapper.h",
                "#include <nice/agent.h>
             #include <nice/interfaces.h>

             #include <stun/stunagent.h>
             #include <stun/stunmessage.h>
             #include <stun/constants.h>
             #include <stun/usages/bind.h>
             #include <stun/usages/ice.h>
             #include <stun/usages/turn.h>
             #include <stun/usages/timer.h>

             #include <nice/pseudotcp.h>",
            )
            // ICE Library
            .whitelist_function("nice_.+")
            .whitelist_type("NICE.+")
            .whitelist_type("_?Nice.+")
            .whitelist_type("_?TurnServer")
            // STUN Library
            .whitelist_function("stun_.+")
            .whitelist_type("STUN.+")
            .whitelist_type("TURN.+")
            .whitelist_type("_?[Ss]tun.+")
            // contains `va_list` type argument which seems like it might not be handled properly
            .opaque_type("StunDebugHandler")
            // Pseudo TCP Socket implementation
            .whitelist_function("pseudo_tcp_.+")
            .whitelist_type("_?PseudoTcp.+")
            // Disable recursive whitelisting, we're using libc, glib-sys, etc.
            .whitelist_recursively(false)
            .clang_args(
                libnice_include_dirs
                    .iter()
                    .map(|path| format!("-I{}", path)),
            )
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
    */
}

fn build_meson(source: &PathBuf, output_path: &PathBuf, configs_promoted: bool) {
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libnice_build");
    println!("Building to {}", build_path.to_str().unwrap());

    /* setup the build */
    {
        let mut compile = Command::new("meson");

        compile.arg("setup");
        compile.arg("--prefix");
        compile.arg(&output_path);
        compile.arg("--default-library");
        compile.arg("static");
        compile.arg("-Dgstreamer=disabled");
        compile.arg("-Dtests=disabled");
        compile.arg(&build_path);
        compile.arg(&source);
        compile.stdout(Stdio::piped());

        let result = compile.spawn().expect("failed to launch meson command")
            .wait_with_output().expect("failed to wait for the process");

        if ! result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);

            println!("Failed to execute command:");
            println!("{}", &stdout);
            if stdout.find("ERROR: Unknown compiler(s):").is_some() {
                panic!("Missing any c compiler. If you're under windows,\
                    ensure the compiler is within the PATH environment variable.");
            } else if stdout.find("meson wrap promote subprojects").is_some() && !configs_promoted {
                let glib_subprojects = PathBuf::from("subprojects").join("glib-2.64.2").join("subprojects");
                for file in [
                    glib_subprojects.join("zlib.wrap"),
                    glib_subprojects.join("libffi.wrap"),
                    glib_subprojects.join("proxy-libintl.wrap"),
                ].iter() {
                    println!("Promoting wrap file {}", file.to_str().unwrap());
                    let succeeded = Command::new("meson")
                        .current_dir(&source)
                        .arg("wrap")
                        .arg("promote")
                        .arg(file)
                        .spawn().expect("failed to spawn meson command")
                        .wait().expect(format!("failed to execute promote command for {}", file.to_str().unwrap()).as_str())
                        .success();
                    if ! succeeded {
                        panic!("Failed to promote {}", file.to_str().unwrap());
                    }
                }

                build_meson(source, output_path, true);
                return;
            }

            panic!("Unknown compiler error");
        }
    }

    /* build the build */
    {
        let mut compile = Command::new("meson");
        compile.arg("compile");
        compile.arg("-C");
        compile.arg(&build_path);

        let success = compile.spawn()
            .expect("failed to spawn meson command")
            .wait().expect("failed to wait for the meson command")
            .success();

        if ! success {
            panic!("Failed to compile libnice.");
        }
    }

    /* install libnice */
    {
        let mut compile = Command::new("meson");
        compile.arg("install");
        compile.arg("-C");
        compile.arg(&build_path);

        let success = compile.spawn()
            .expect("failed to spawn meson command")
            .wait().expect("failed to wait for the meson command")
            .success();

        if ! success {
            panic!("Failed to install libnice.");
        }
    }
}
