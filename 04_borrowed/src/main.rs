fn main() {
    let world = "world";
    println!("{}", greet(world));
    println!("{}", greet(world));
}

fn greet(who: &str) -> String {
    format!("Hello, {}!", who)
}
