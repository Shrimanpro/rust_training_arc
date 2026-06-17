fn main() {
    let mut temp: String = String::new();
    let _ = std::io::stdin().read_line(&mut temp);
    let mut input = temp.split_whitespace();
 
    let mut op;
    let mut done: bool = false;
    while !done
    {
        op = input.next();
        match op 
        {
            Some(test) => println!("{}", test),
            None => done = true,
        };

    }

}
