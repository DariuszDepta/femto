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
    println!("All test suites completed successfully for version: {}", VERSION);
}

fn main() {
    run();
}

#[test]
fn test_main() {
    run();
}
