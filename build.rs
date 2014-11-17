#![feature(slicing_syntax)]
use std::os;
use std::io::Command;
use std::io::File;
use std::io::stdio::stderr;

fn fail(c: &Command) -> ! {
    panic!("command failed: {}", c);
}


fn get_output(c: &mut Command) -> Vec<u8> {
    //writeln!(stderr(), "{}", c);
    let o = c.output().unwrap_or_else(|_| fail(c));
    if !o.status.success() { fail(c); }
    let mut out = o.output;
    out.pop();
    out
}

fn check(c: &mut Command) {
    //writeln!(stderr(), "{}", c);
    let s = c.status().unwrap_or_else(|_| fail(c));
    if !s.success() { fail(c); }
}


// this should do quotes in future
fn split(x: &Vec<u8>) -> Vec<&[u8]> {
    x.split(|c| *c == b' ').filter(|x| x.len() != 0).collect()
}

// why am i finding myself dealing with memory management in a *build script*
fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let llvm_config = os::getenv("LLVM_CONFIG").unwrap_or("llvm-config".into_string());
    let mut ldflags = get_output(Command::new(&llvm_config).args(["--ldflags", "--libs", "--system-libs"]));
    ldflags = ldflags.iter().map(|&x| if x == b'\n' { b' ' } else { x }).collect();
    let cflags = get_output(Command::new(&llvm_config).arg("--cflags"));
    let cxxflags = get_output(Command::new(&llvm_config).arg("--cxxflags"));
    let bq_o = dst.join("bq.o");
    let libbq_a = dst.join("libbq.a");
    let temp_rs = dst.join("temp.rs");
    check(Command::new("c++").args(split(&cxxflags)[]).args(["-O3", "-c", "-o"]).arg(&bq_o).arg("src/bq.cpp"));
    check(Command::new("ar").arg("rcs").arg(libbq_a).arg(&bq_o));
    check(Command::new("bindgen").args(split(&cflags)[]).args(["-match", "llvm", "-o"]).arg(temp_rs).arg("src/autollvm.h"));
    let link_args = dst.join("link_args.rs");
    let mut f = File::create(&link_args).unwrap();
    f.write_str("#[link_args = \"").unwrap();
    f.write(ldflags[]).unwrap();
    f.write_str("\"]\n").unwrap();
}
