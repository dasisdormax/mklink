mod args;

use std::ffi::{OsString, OsStr, c_void};
use std::os::windows::prelude::*;
use std::iter::once;
use std::process::exit;

use clap::Parser;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{CreateSymbolicLinkW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW};

fn to_unicode<T: AsRef<OsStr>>(s: T) -> Vec<u16> {
    OsStr::new(&s).encode_wide().chain(once(0)).collect()
}

const SYMBOLIC_LINK_FLAG_FILE: u32 = 0x0;
const SYMBOLIC_LINK_FLAG_DIRECTORY: u32 = 0x1;
const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: u32 = 0x2;

fn main() {
    let args = args::Args::parse();
    let symlink_file_name: Vec<u16> = to_unicode(args.get_link());
    let target_file_name = to_unicode(args.get_target());

    let mut flags = 0u32;
    flags |= SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE;
    flags |= if args.is_directory() {
        SYMBOLIC_LINK_FLAG_DIRECTORY
    } else {
        SYMBOLIC_LINK_FLAG_FILE
    };
    let code = unsafe {
        let result = CreateSymbolicLinkW(
            symlink_file_name.as_ptr(), 
            target_file_name.as_ptr(), 
            flags
        );
        if result == 0 {
            GetLastError()
        } else {
            return;
        }
    };
    let mut buf = [0u16; 4096];
    let len = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            0 as *const c_void,
            code,
            0,
            &mut buf as *mut u16,
            4096,
            0 as *mut *mut i8
        )
    } as usize;
    let msg = OsString::from_wide(&buf[0..len]);
    let s = msg.to_str().unwrap();
    eprint!("{}", s);
    exit(1);
}
