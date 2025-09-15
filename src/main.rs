use coin_flip::*;
use std::env;
fn main() {
    /*
    Method #1:

    let total_runs: u64;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg = args.iter().skip(1).nth(0).unwrap();
        let Ok(a) = first_arg.parse::<u64>() else {
            panic!("Invalid number given.");
        };
        total_runs = a;
    } else {
        println!(
            "No provided run count, defaulting to 100.\nRun with 'cargo run <number of runs>' to test with a custom number of runs."
        );
        total_runs = 100;
    }

    */

    // Method #2:
    let default_runs: u64 = 100;

    let total_runs: u64 = env::args()
        .nth(1)
        .map(|arg| {
            arg.parse::<u64>().unwrap_or_else(|_| {
                panic!("Invalid number given. Expected a positive integer, got '{}'", arg)
            })
        })
        .unwrap_or_else(|| {
            println!(
                "No provided run count, defaulting to {}.\nRun with 'cargo run <number of runs>' to test with a custom number of runs.",
                default_runs
            );
            default_runs
        });
    // End of Method 2

    println!("Running with {} runs.", total_runs);
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
