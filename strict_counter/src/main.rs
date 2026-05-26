use rand::Rng;

fn main() 
{
    let daily_goal: u16 =  rand::thread_rng().gen_range(6000..=12000);
    println!("Hello, world! {daily_goal}");
}           
