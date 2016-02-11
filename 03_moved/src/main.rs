fn main() {
    let world = "world".to_string();
    println!("{}", greet(world));
    println!("{}", greet(world));
}

fn greet(who: String) -> String {
    format!("Hello, {}!", who)
}
