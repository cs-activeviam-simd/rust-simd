use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    let mut status = Command::new("gcc").args(&["src/libSIMD.c", "-c", "-march=native", "-O3", "-o"])
                       .arg(&format!("{}/libSIMD.o", out_dir))
                       .status().unwrap();
    assert!(status.success());
    status = Command::new("ar").args(&["crus", "liblibSIMD.a", "libSIMD.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    assert!(status.success());

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=libSIMD");
}
