extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use build_utils::build::{LibraryType, BuildStepError, MesonBuild};
use build_utils::source::BuildSourceGit;
use build_utils::{BuildStep, BuildResult, Build, execute_build_command};
use std::hash::{Hasher, Hash};
use build_utils::resolve_env_var;

struct MesonPromote {
    files: Vec<String>
}

impl BuildStep for MesonPromote {
    fn name(&self) -> &str {
        "meson promote"
    }

    fn hash(&self, hasher: &mut Box<dyn Hasher>) {
        self.files.hash(hasher);
    }

    fn execute(&mut self, build: &Build, _result: &mut BuildResult) -> Result<(), BuildStepError> {
        let source_dir = build.source().local_directory();

        for file in self.files.iter() {
            println!("Promoting wrap file {}", file);
            let mut command = Command::new("meson");
            command.current_dir(&source_dir)
                    .arg("wrap")
                    .arg("promote")
                    .arg(source_dir.join(file).to_str().unwrap());

            execute_build_command(&mut command, format!("failed to execute promote command for {}", file).as_str())?;
        }

        Ok(())
    }
}

fn main() {
    let build_name = "libnice";
    let source = BuildSourceGit::builder("https://github.com/WolverinDEV/libnice.git".to_owned())
        .revision(Some("5dff8876ca93adeadc1381895dba6536bdc21b0c".to_owned()))
        .build();

    let meson = MesonBuild::builder()
        .promote_callback(|source| {
            println!("Callback promote for {:?}", source);
            vec![
                "subprojects/glib-2.64.2/subprojects/zlib.wrap".to_owned(),
                "subprojects/glib-2.64.2/subprojects/libffi.wrap".to_owned(),
                "subprojects/glib-2.64.2/subprojects/proxy-libintl.wrap".to_owned()
            ]
        })
        .meson_option("gstreamer", "disabled")
        .meson_option("tests", "disabled")
        .meson_option("examples", "disabled")
        .meson_option("crypto-library", "openssl")
        .meson_option("gupnp", resolve_env_var!(build_name, "gupnp").unwrap_or("auto".to_owned()))
        .build();

    let mut build_builder = Build::builder()
        .name(build_name)
        .source(Box::new(source))
        .add_step(Box::new(meson))
        .remove_build_dir(false);

    match build_builder.build().expect("failed to generate build").execute() {
        Ok(mut result) => {
            if cfg!(windows) {
                /* required by libnice */
                println!("cargo:rustc-link-lib=dylib=bcrypt");
                println!("cargo:rustc-link-lib=dylib=Iphlpapi");
            }

            result.emit_cargo();

            /* TODO: Generate bindings */
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
        },
        Err(error) => {
            println!("{}", error.pretty_format());
            panic!("failed to execute libnice build");
        }
    }
}