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

fn main() {
    let me = User {
        name: "Rasmus Kaj".to_string(),
        username: "kaj".to_string()
    };
    println!("I am {}.", me);
    println!("My data is {:?}.", me);
    println!("{:#x} + {:#o} is {:x}", 256, 64, 256 + 64);
}
