extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let libc_dir: PathBuf = match env::var_os("LIBC_DIR") {
        Some(libc) => libc.into(),
        None => {
            let mut lib_path = PathBuf::from(env::var("HOME").unwrap());
            lib_path.push("riscv32im-linux-x86_64/");
            lib_path
        }
    };

    let plibc = libc_dir.join("picolibc/riscv32-unknown-elf/lib/");

    let include = libc_dir.join("picolibc/riscv32-unknown-elf/include/");
    // let libc = plibc.join("lib/");
    let sys_include = libc_dir.join("picolibc/riscv32-unknown-elf/sys-include/");
    let toolchain = libc_dir.join("riscv32-unknown-elf/");

    let blip_buf_dir = PathBuf::from("./");

    let mut cc_config = cc::Build::new();

    cc_config
        .compiler("clang")
        .include(&include)
        .include(&sys_include)
        //.include(&blip_buf_dir)
        .include(&toolchain)
        .target("riscv32-unknown-elf")
        .flag(&format!("--gcc-toolchain={}", libc_dir.to_str().unwrap()) as &str);

    let target_features = "-a -c +m".split(" ");
    for flag in target_features {
        let tokens = format!("-Xclang -target-feature -Xclang {}", flag);
        for token in tokens.split(" ") {
            cc_config.flag(token);
        }
    }

    println!("cargo:rustc-link-lib=static={}", "c");
    println!("cargo:rustc-link-search=native={}", plibc.display());
    println!("cargo:rerun-if-env-changed=LIBC_DIR");

    cc_config
        .file("blip_buf.c")
        .compile("blip_buf.a");
}
