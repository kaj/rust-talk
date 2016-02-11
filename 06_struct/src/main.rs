struct User {
    pub name: String,
    pub username: String
}

fn main() {
    let me = User {
        name: "Rasmus Kaj".to_string(),
        username: "kaj".to_string()
    };
    println!("I am {} <{}@kth.se>", me.name, me.username);
}
