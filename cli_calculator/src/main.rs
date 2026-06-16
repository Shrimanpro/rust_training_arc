fn main() {
    println!("Hello, world!");
    let mut temp: String = String::new();
    let _ = std::io::stdin().read_line(&mut temp);
    let mut t = temp.split_whitespace();
    let s = t.next();

    if let Some(val) = s 
    {
        println!("{}", val);

    }
    else 
    {
        println!("er");
    }
    // println!("{}", s);
}
