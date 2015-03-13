#![feature(path, io, std_misc)]
#![allow(plugin_as_library)]
use std::process::Command;
use std::fs::File;
use std::io::Write;
//use std::io::stderr;
use std::os::unix::OsStrExt;
use std::ffi::OsStr;
use std::path::Path;
use std::env;
//use std::os::unix::AsRawFd;

extern crate bindgen;
//extern crate libc;

fn fail(c: &Command) -> ! {
    panic!("command failed: {:?}", c);
}

/*
fn get_stderr_dammit() {
    if let Ok(file) = File::create("/dev/tty") {
        unsafe { libc::dup2(file.as_raw_fd(), 2); }
    }
}
*/

fn get_output(c: &mut Command) -> Vec<u8> {
    let o = c.output().unwrap_or_else(|_| fail(c));
    if !o.status.success() { fail(c); }
    let mut out = o.stdout;
    while out.last() == Some(&b'\n') { out.pop(); }
    out
}

fn check(c: &mut Command) {
    //writeln!(&mut stderr(), "{:?}", c).unwrap();
    let s = c.status().unwrap_or_else(|_| fail(c));
    if !s.success() { fail(c); }
}


// this should do quotes in future
fn split(x: &Vec<u8>) -> Vec<&OsStr> {
    x.split(|&c| c == b' ' || c == b'\n')
     .filter(|arg| arg.len() != 0)
     .map(|arg| OsStr::from_bytes(arg))
     .collect()
}

// why am i finding myself dealing with memory management in a *build script*
fn main() {
    //get_stderr_dammit();
    let dst = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&dst);
    let llvm_config = env::var("LLVM_CONFIG").unwrap_or("llvm-config".to_string());
    let mut ldflags = get_output(Command::new(&llvm_config)
                                 .args(&["--ldflags", "--libfiles", "--system-libs"]));
    //let ldflags = String::from_utf8(ldflags).unwrap().replace("\n", " ");
    ldflags = ldflags.iter().map(|&x| if x == b'\n' { b' ' } else { x }).collect();
    let cflags = get_output(Command::new(&llvm_config).arg("--cflags"));
    let cxxflags = get_output(Command::new(&llvm_config).arg("--cxxflags"));
    let bq_so = dst.join("libbq.dylib");
    //let libbq_a = dst.join("libbq.a");
    let temp_rs = dst.join("temp.rs");
    check(Command::new("c++")
          .args(&split(&cxxflags))
          .args(&["-dynamiclib", "-Wl,-all_load", "-O3", "-o"])
          .arg(&bq_so)
          .arg("src/bq.cpp")
          .args(&split(&ldflags)));
    /*
    check(Command::new("ar")
          .arg("rcs")
          .arg(&libbq_a)
          .arg(&bq_so));
          */
    let mut options: bindgen::BindgenOptions = std::default::Default::default();
    options.match_pat.push("llvm".to_string());
    options.clang_args.push("src/autollvm.h".to_string());
    for arg in split(&cflags) {
        options.clang_args.push(arg.to_str().unwrap().to_string());
    }
    let bindout = bindgen::Bindings::generate(&options, None, None).unwrap().to_string();
    //let bindout = bindout.replace("extern \"C\" {", format!("#[link_args = \"{}\"]\nextern \"C\" {{", ldflags));
    let mut f = File::create(&temp_rs).unwrap();
    f.write_all(bindout.as_bytes()).unwrap();
    println!("cargo:rustc-flags=-L {} -l bq:dylib", dst.to_str().unwrap());
}
