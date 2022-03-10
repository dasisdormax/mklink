mod args;
use std::os::windows::prelude::OsStrExt;

use clap::Parser;
use winapi::um::winbase::CreateSymbolicLinkW;
use std::iter::once;
use std::ffi::OsStr;

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
    flags |= if(args.is_directory()) {
        SYMBOLIC_LINK_FLAG_DIRECTORY
    } else {
        SYMBOLIC_LINK_FLAG_FILE
    };
    unsafe {
        CreateSymbolicLinkW(
            symlink_file_name.as_ptr(), 
            target_file_name.as_ptr(), 
            flags
        );
    }
}
