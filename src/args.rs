use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    /// Indicates that the link points to a directory instead of a regular file
    #[clap(short='d')]
    flag_dir: bool,

    /// The name of the symbolic link
    link: String,

    /// The path that the link points to
    /// 
    /// Can either be an absolute path or relative to the directory containing the link.
    target: String,
}

impl Args {
    pub fn is_directory(&self) -> bool {
        self.flag_dir
    }

    pub fn get_target(&self) -> &str {
        &self.target
    }

    pub fn get_link(&self) -> &str {
        &self.link
    }
}