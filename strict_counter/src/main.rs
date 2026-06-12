use rand::Rng;

fn main() 
{
    let mut comp_goals: u8 = 0;
    let total_days: u8 = 7;
    for _i in 1..=total_days
    {
        let daily_goal: u16 = rand::thread_rng().gen_range(6000..=12000);
        println!("Today's daily goal is {daily_goal}:");
        let mut steps: u16 = 0;
        stepping(&mut steps);
        println!("{steps} steps walked.");
        if steps >= daily_goal 
        {
            comp_goals += 1;
        }
    }
    println!("{comp_goals}/{total_days} days goal has met.");
    let percent: f32 = f32::from(comp_goals) / f32::from(total_days) * 100.0;
    println!("Total completion percentage is {percent}.")
}

fn stepping(s: &mut u16)
{
    *s += rand::thread_rng().gen_range(6000..=12000);
}
