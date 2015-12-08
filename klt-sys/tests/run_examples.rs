extern crate klt_sys as klt;
use std::process::Command;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::fmt::{Display, Debug};

macro_rules! s {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const raw::c_char
    }
}

macro_rules! idx {
    ($strukt:ident . $arr:ident [ $idx:expr, $n:ident ] . $member:ident) => {
        (*slice::from_raw_parts($strukt.$arr, $strukt.$n as usize)[$idx as usize]).$member
    }
}

trait Complain<T> {
    fn complain<M: Display>(self, msg: M) -> T;
}

impl<T, E: Debug> Complain<T> for Result<T, E> {
    fn complain<M: Display>(self, msg: M) -> T {
        self.unwrap_or_else(|e| {
            panic!("{}: {:?}", msg, e)
        })
    }
}

impl<T> Complain<T> for Option<T> {
    fn complain<M: Display>(self, msg: M) -> T {
        self.expect(&format!("{}", msg))
    }
}

fn cp(name: &str) -> io::Result<()> {
    println!("copying {}", name);

    try!(fs::copy(Path::new("../lib/klt/dummy").with_file_name(name), name));
    Ok(())
}

fn diff(name: &str) -> io::Result<()> {
    println!("diffing {}", name);
    
    let mut s1 = Vec::new();
    try!(try!(File::open(Path::new("../lib/klt/dummy").with_file_name(name))).read_to_end(&mut s1));

    let mut s2 = Vec::with_capacity(s1.len());
    try!(try!(File::open(name)).read_to_end(&mut s2));

    if s1 != s2 {
        panic!("{} has incorrect contents", name);
    }
    
    Ok(())
}

fn cd(comps: &[&str]) -> io::Result<PathBuf> {
    let prev = env::current_dir().complain("no current dir");
    let mut p = Path::new(file!()).parent().complain("filename has no parent dir").to_owned();

    for c in comps {
        p.push(c);
    }

    println!("cd {:?}", p);
    try!(env::set_current_dir(p));
    Ok(prev)
}

mod example1;
mod example2;
mod example3;
mod example4;
mod example5;

fn do_example(bin: &str, inputs: &[&str], outputs: &[&str], f: unsafe fn()) {
    let prev = cd(&["..", "lib", "klt"]).complain("could not chdir to klt dir");

    Command::new(bin).status().complain("C example failed");

    env::set_current_dir(&prev).complain("could not chdir to root dir");
    cd(&[]).complain("could not chdir to tests dir");

    for f in inputs {
        cp(f).complain("could not copy in test images");
    }

    unsafe {
        f();
    }

    for f in outputs {
        diff(f).complain("could not compare outputs");
    }

    env::set_current_dir(&prev).complain("could not chdir to root dir");
}

#[test]
fn ex1() {
    do_example("./example1",
               &["img0.pgm", "img1.pgm"],
               &["feat1.ppm", "feat2.ppm", "feat1.txt", "feat2.txt", "feat2.fl"],
               example1::unsafe_main);
}

#[test]
fn ex2() {
    do_example("./example2",
               &["img0.pgm", "img1.pgm"],
               &["feat1.ppm", "feat2.ppm", "feat1.txt", "feat2.txt"],
               example2::unsafe_main);
}

#[test]
fn ex3() {
    do_example("./example3",
               &["img0.pgm", "img1.pgm", "img2.pgm", "img3.pgm", "img4.pgm", "img5.pgm", "img6.pgm", "img7.pgm", "img8.pgm", "img9.pgm"],
               &["feat0.ppm", "feat1.ppm", "feat2.ppm", "feat3.ppm", "feat4.ppm", "feat5.ppm", "feat6.ppm", "feat7.ppm", "feat8.ppm", "feat9.ppm",
                 "features.txt", "features.ft"],
               example3::unsafe_main);
}

#[test]
fn ex4() {
    do_example("./example4",
               &[], // uses features.txt from previous test
               &["feat1.txt", "ft2.txt", "ft3.txt"],
               example4::unsafe_main);
}

#[test]
fn ex5() {
    do_example("./example5",
               &["img0.pgm", "img2.pgm"],
               &["feat1b.ppm", "feat2b.ppm"],
               example5::unsafe_main);
}

