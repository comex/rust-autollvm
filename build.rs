#![allow(plugin_as_library)]

/*
#![feature(libc)]
extern crate libc;
use std::os::unix::prelude::AsRawFd;
fn get_stderr_dammit() {
    if let Ok(file) = File::create("/dev/tty") {
        unsafe { libc::dup2(file.as_raw_fd(), 2); }
    }
}
*/

use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::io::stderr;
use std::os::unix::ffi::OsStrExt;
use std::ffi::OsStr;
use std::path::Path;
use std::env;

extern crate bindgen;

fn fail(c: &Command) -> ! {
    panic!("command failed: {:?}", c);
}

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

#[derive(Debug)]
struct StdLogger;

impl bindgen::Logger for StdLogger {
    fn error(&self, msg: &str) {
        writeln!(stderr(), "{}", msg).unwrap();
    }

    fn warn(&self, msg: &str) {
        writeln!(stderr(), "{}", msg).unwrap();
    }
}


// why am i finding myself dealing with memory management in a *build script*
fn main() {
    //get_stderr_dammit();
    let dst = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&dst);
    let llvm_config = env::var("LLVM_CONFIG").unwrap_or("llvm-config".to_string());
    let ldflags = get_output(Command::new(&llvm_config)
                             .args(&["--ldflags", "--libs", "--system-libs"]));
    let ldflags = String::from_utf8(ldflags).unwrap().replace("\n", " ");
    // hack: without this I get linker errors due to missing -ledit
    let ldflags = ldflags.replace("-lLLVMLineEditor ", "");
    let cflags = get_output(Command::new(&llvm_config).arg("--cflags"));
    let cxxflags = get_output(Command::new(&llvm_config).arg("--cxxflags"));
    let bq_o = dst.join("bq.o");
    let libbq_a = dst.join("libbq.a");
    let temp_rs = dst.join("temp.rs");
    check(Command::new("c++")
          .args(&split(&cxxflags))
          .args(&["-O3", "-c", "-o"])
          .arg(&bq_o)
          .arg("src/bq.cpp"));
    check(Command::new("ar")
          .arg("rcs")
          .arg(&libbq_a)
          .arg(&bq_o));
    let mut options: bindgen::BindgenOptions = std::default::Default::default();
    options.match_pat.push("llvm".to_string());
    options.clang_args.push("src/autollvm.h".to_string());
    for arg in split(&cflags) {
        options.clang_args.push(arg.to_str().unwrap().to_string());
    }
    //writeln!(&mut stderr(), "{:?}", options.clang_args).unwrap();
    let bindings = bindgen::Bindings::generate(&options, Some(&StdLogger as &bindgen::Logger), None).unwrap();
    let bindout = bindings.to_string();
    if bindout.len() == 0 { panic!("bindgen r dumb"); }
    let mut bindout_without_modattrs = &bindout[..];
    if let Some(x) = bindout.find("#![") {
        if let Some(y) = bindout[x..].find(']') {
            bindout_without_modattrs = &bindout[x+y+1..];
        }
    }


    let mut f = File::create(&temp_rs).unwrap();
    f.write_all(bindout_without_modattrs.as_bytes()).unwrap();
    f.write_all(format!("#[link(name = \"c++\")]\n#[link_args = \"-all_load {}\"]\nextern \"C\" {{}}", ldflags).as_bytes()).unwrap();
    //bindings.write(Box::new(f)).unwrap(); // x.x
    println!("cargo:rustc-flags=-L {} -l static=bq", dst.to_str().unwrap());
}
