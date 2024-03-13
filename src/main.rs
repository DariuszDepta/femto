mod contracts;
mod suites;

fn run() {
    suites::run();
    println!("All test suites completed successfully.");
}

fn main() {
    run();
}

#[test]
fn test_main() {
    run();
}
