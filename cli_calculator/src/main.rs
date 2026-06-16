fn main() {
    println!("Hello, world!");
    let mut temp: String = String::new();
    let _ = std::io::stdin().read_line(&mut temp);
    let mut t = temp.split_whitespace();
    let s = t.next();
    // println!("{}", s);
}
