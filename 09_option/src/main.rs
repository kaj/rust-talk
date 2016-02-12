use std::fmt;

#[derive(Debug)]
struct User {
    pub name: String,
    pub username: String
}

impl fmt::Display for User {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{} <{}@kth.se>", self.name, self.username)
    }
}

fn getuser(id: &str) -> Option<User> {
    match id {
        "kaj" => Some(User {
            name: "Rasmus Kaj".to_string(),
            username: "kaj".to_string()
        }),
        _ => None
    }
}

fn main() {
    for username in vec!("gurgel", "kaj", "blargel") {
        if let Some(user) = getuser(username) {
            println!("{} is {}.", username, user);
        } else {
            println!("{} does not exist.", username);
        }
    }

    println!("My data is {:?}", getuser("kaj"));
    println!("Nobodys data is {:?}", getuser("x"));
}
