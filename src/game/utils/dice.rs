use rand::Rng;

pub fn roll_dice(num_dice: i32) -> Vec<i32>{
    let mut rng = rand::thread_rng();
    let mut results = Vec::new();

    for _ in 0..num_dice{
        let roll = rng.gen_range(1..=6);
        results.push(roll);
    }

    results 
}

pub fn count_successes(rolls: & Vec<i32>) -> i32 {
    rolls.iter().filter(|&&x| x >=5).count() as i32
}