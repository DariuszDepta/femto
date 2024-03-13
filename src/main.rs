mod contracts;
mod suites;

#[cfg(feature = "v_0_1_1")]
const VERSION: &str = "v-0-1-1";

#[cfg(feature = "v_0_1_2")]
const VERSION: &str = "v-0-1-2";

#[cfg(feature = "v_0_1_3")]
const VERSION: &str = "v-0-1-3";

#[cfg(feature = "v_0_1_4")]
const VERSION: &str = "v-0-1-4";

#[cfg(feature = "v_1_1_1")]
const VERSION: &str = "v-1-1-1";

#[cfg(feature = "v_1_1_2")]
const VERSION: &str = "v-1-1-2";

#[cfg(feature = "v_1_1_3")]
const VERSION: &str = "v-1-1-3";

#[cfg(feature = "v_1_1_4")]
const VERSION: &str = "v-1-1-4";

#[cfg(feature = "v_2")]
const VERSION: &str = "v-2";

fn run() {
    suites::run();
    println!("┌───────────────────────────────────────────────────────────────┐");
    println!(
        "│ \u{1b}[32;1mAll tests completed successfully for version: \u{1b}[33;1m{VERSION:15}\u{1b}[0m │",
    );
    println!("└───────────────────────────────────────────────────────────────┘");
}

fn main() {}

#[test]
fn test_main() {
    run();
}
