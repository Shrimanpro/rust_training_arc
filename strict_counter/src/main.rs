use rand::Rng;

fn main() 
{
    for _i in 1..=7
    {
        let daily_goal: u16 = rand::thread_rng().gen_range(6000..=12000);
        println!("Today's daily goal is {daily_goal}");
        let mut steps: u16 = 0;
        println!("{steps}");
        stepping(&mut steps);
        println!("{steps}");
    }
}

fn stepping(s: &mut u16)
{
    *s += rand::thread_rng().gen_range(6000..=12000);
}
