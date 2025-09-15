use coin_flip::*;
fn main() {
    let total_runs: u64 = 100;
    let mut heads_results: u32 = 0;
    let mut tails_results: u32 = 0;
    let mut rng = rand::rng();
    let mut handler = |r: &Coin| {
        if *r == Coin::Heads {
            heads_results += 1;
        } else {
            tails_results += 1;
        }
    };
    for _ in 1..=total_runs {
        flip_coin(&mut rng, &mut handler);
    }
    let heads_percentage = calculate_percentage(total_runs, heads_results);
    let tails_percentage = calculate_percentage(total_runs, tails_results);
    println!(
        "Out of {total_runs}:\nHeads: {heads_results} (%{:.2})\nTails: {tails_results} (%{:.2})",
        heads_percentage, tails_percentage
    );
}
