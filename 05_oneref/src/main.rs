
fn main() {
    let mut x = vec!["Hello", "world"];
    let y = &x[0];
    // use y ...
    x.push("foo"); // nope, y borrows x
}
