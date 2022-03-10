use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    /// Indicates that the file points to a directory instead of a regular file
    #[clap(short='d')]
    is_directory: bool,

    /// The location the link points to
    /// 
    /// Can either be an absolute path or relative to the directory containing the link.
    target: String,

    /// The location the link is created at
    link: String,
}