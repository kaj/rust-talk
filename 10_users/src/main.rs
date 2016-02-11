extern crate users;
use std::path::PathBuf;
use std::fs::metadata;
use users::get_user_by_name;
use std::io::{Result, Error, ErrorKind};
use users::os::unix::UserExt;

fn homedir(username: &str) -> Result<PathBuf> {
    if let Some(user) = get_user_by_name(username) {
        Ok(user.home_dir().to_path_buf())
    } else {
        Err(Error::new(ErrorKind::Other, "no such user"))
    }
}

fn projectsize(username: &str) -> Result<u64> {
    let home = try!(homedir(username));
    let meta = try!(metadata(home));
    Ok(meta.len())
}

fn main() {
    for username in vec!("ftp", "kaj", "blargel") {
        match projectsize(username) {
            Ok(size) => println!("Home of {} is {} bytes", username, size),
            Err(e) => println!("Error for {}: {}", username, e)
        }
    }
}
