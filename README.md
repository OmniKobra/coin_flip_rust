# 🪙 Coin Flip Simulator

A simple Rust program that simulates flipping a fair coin multiple times and reports the results with percentages.

---

## ✨ Features
- Simulates flipping a coin (`Heads` or `Tails`).
- Keeps track of the number of times each side comes up.
- Calculates percentages of outcomes.
- Uses closures to handle coin flip events flexibly.

---

## 📦 Requirements
- [Rust](https://www.rust-lang.org/) (edition 2021 or newer)
- [rand](https://crates.io/crates/rand) crate

Add `rand` to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

---

## ▶️ Usage
Clone the repository and run the program:

```bash
cargo run
```

Example output:

```
Out of 100:
Heads: 50 (50.00%)
Tails: 50 (50.00%)
```

---

## 🛠 Project Structure
```
src/
├── lib.rs   # Library code (coin enum, flip logic, percentage calculation)
└── main.rs  # Entry point, runs the simulation
```

---

## 📖 How It Works
1. `flip_coin` generates a random `Heads` or `Tails`.
2. A closure (handler) processes the result (e.g., counting outcomes).
3. After all runs, the program calculates and prints percentages.

---

## 🔮 Possible Improvements
- Make the number of runs configurable via CLI args.
- Add support for multiple coins at once.
- Write unit tests for the coin flip logic.
- Provide a histogram of results for large runs.

---

## 📜 License
This project is open source and available under the [MIT License](LICENSE).
