use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
/// A standalone cmd-like mklink implementation
/// 
/// When invoked without arguments, this starts 'batch mode' where you can send commands through stdin.
pub struct Args {
    /// Indicates that the link points to a directory instead of a regular file
    #[clap(short='d', requires = "link")]
    flag_dir: bool,

    /// The name of the symbolic link
    #[clap(requires = "target")]
    link: Option<String>,

    /// The path that the link points to
    /// 
    /// Can either be an absolute path or relative to the directory containing the link.
    target: Option<String>,
}

impl Args {
    pub fn is_directory(&self) -> bool {
        self.flag_dir
    }

    pub fn get_target(&self) -> Option<&str> {
        self.target.as_ref().map(|it| it.as_str())
    }

    pub fn get_link(&self) -> Option<&str> {
        self.link.as_ref().map(|it| it.as_str())
    }
}