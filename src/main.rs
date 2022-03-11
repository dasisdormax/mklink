mod args;

use std::ffi::{OsString, OsStr, c_void};
use std::io;
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

fn symlink(link: &str, target: &str, is_dir: bool) {
    let symlink_file_name: Vec<u16> = to_unicode(link);
    let target_file_name = to_unicode(target);

    let mut flags = 0u32;
    flags |= SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE;
    flags |= if is_dir {
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

fn decode_line(line: &str) -> Vec<String> {
    let mut args = Vec::<String>::new();
    let mut current = String::new();
    let mut curr_quote = ' ';
    let mut is_backslashed= false;
    let mut is_word = false;
    for c in line.chars() {
        if c.is_whitespace() && curr_quote == ' ' {
            if is_word {
                args.push(current.clone());
                current.clear();
                is_word = false;
            }
            continue;
        }
        is_word = true;
        if curr_quote == c {
            curr_quote = ' ';
            continue;
        }
        match c {
            '\''|'"' => {
                if curr_quote == ' ' && !is_backslashed {
                    curr_quote = c;
                    continue;
                }
            }
            '\\' => {
                if !is_backslashed {
                    is_backslashed = true;
                    continue;
                }
            }
            _ => {
                if is_backslashed {
                    current.push('\\');
                }
            }
        }
        current.push(c);
        is_backslashed = false;
    }
    if is_word {
        args.push(current.clone());
        current.clear();
    }
    args
}

fn listen() -> ! {
    let mut line = String::new();
    while let Ok(bytes) = io::stdin().read_line(&mut line) {
        if bytes == 0 {
            exit(0);
        }
        let args = decode_line(line.as_str());
        line.clear();

        let mut ia = args.iter();
        if args.len() < 3 {
            eprintln!("Usage: mklink [-d | /D] <LINK> <TARGET>");
            continue;
        }
        ia.next().unwrap();
        let mut link: &String = ia.next().unwrap();
        let is_dir = link == "-d" || link.eq_ignore_ascii_case("/d");
        if is_dir {
            link = ia.next().unwrap();
        }
        let target = match ia.next() {
            Some(x) => x,
            None => {
                eprintln!("Usage: mklink [-d | /D] <LINK> <TARGET>");
                continue;
            }
        };
        symlink(link.as_str(), target.as_str(), is_dir);
    }
    exit(1)
}

fn main() {
    let args = args::Args::parse();
    let link = args.get_link().unwrap_or_else(|| listen());
    let target = args.get_target().unwrap();
    symlink(link, target, args.is_directory())
}
