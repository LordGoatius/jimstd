use core::{slice, str};
use std::{
    ffi::{CStr, c_char, c_int}, sync::OnceLock
};

// Primarily pulled from the rust `argv` crate, but only for how to access argc and argv through rust
static ARGC: OnceLock<c_int> = OnceLock::new();
static ARGV: OnceLock<Argv> = OnceLock::new();
static ENVP: OnceLock<Envp> = OnceLock::new();

#[repr(transparent)]
#[derive(Debug)]
pub struct Argv(*const *const c_char);

#[repr(transparent)]
#[derive(Debug)]
pub struct Envp(*const *const c_char);

unsafe impl Send for Argv {}
unsafe impl Sync for Argv {}

unsafe impl Send for Envp {}
unsafe impl Sync for Envp {}

#[unsafe(link_section = ".init_array")]
#[used]
static INIT: unsafe extern "C" fn(c_int, *const *const c_char, *const *const c_char) = capture;

extern "C" fn capture(argc: c_int, argv: *const *const c_char, envp: *const *const c_char) {
    ARGC.set(argc).expect("Unable to initalize argc");
    ARGV.set(Argv(argv)).expect("Unable to initalize argv");
    ENVP.set(Envp(envp)).expect("Unable to initalize envp");
}

#[repr(C)]
pub struct Args {
    argc: usize,
    argv: &'static [*const c_char],
}

#[repr(C)]
pub struct ArgsIter {
    argc: usize,
    argv: &'static [*const c_char],
    curr: usize,
}

pub fn args() -> Args {
    let argc = *ARGC.get().unwrap() as usize;
    Args {
        argc,
        argv: unsafe {
            slice::from_raw_parts(ARGV.get().unwrap().0.cast(), argc)
        },
    }
}

impl Args {
    pub fn iter(&self) -> ArgsIter {
        let &Args { argc, argv } = self;
        ArgsIter { argc, argv, curr: 0 }
    }

    pub fn len(&self) -> usize {
        self.argc
    }

    pub fn get(&self, index: usize) -> &'static str {
        self.argv.get(index).and_then(|&ptr| unsafe {
            let cstr = CStr::from_ptr(ptr);
            cstr.to_str().ok()
        }).unwrap()
    }
}

impl Iterator for ArgsIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr += 1;
        self.argv.get(curr).and_then(|&ptr| unsafe {
            let cstr = CStr::from_ptr(ptr);
            cstr.to_str().ok()
        })
    }
}

impl ExactSizeIterator for ArgsIter {
    fn len(&self) -> usize {
        self.argc
    }
}
