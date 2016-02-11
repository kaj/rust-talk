use std::fmt;

#[derive(Debug)]
enum User {
    Noone,
    Someone {
        name: String,
        username: String
    }
}

impl fmt::Display for User {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &User::Noone => write!(out, "nobody"),
            &User::Someone{ref name, ref username} =>
                write!(out, "{} <{}@kth.se>", name, username)
        }
    }
}

fn getuser(id: &str) -> User {
    match id {
        "kaj" => User::Someone {
            name: "Rasmus Kaj".to_string(),
            username: "kaj".to_string()
        },
        _ => User::Noone
    }
}

fn main() {
    for username in vec!("gurgel", "kaj", "blargel") {
        println!("{} is {}.", username, getuser(username));
    }
}
