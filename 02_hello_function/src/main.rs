fn main() {
    println!("{}", greet("world"));
}

fn greet(who: &str) -> String {
    format!("Hello, {}!", who)
}
