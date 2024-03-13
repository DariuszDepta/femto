mod contracts;
mod suites;

fn run() {
    suites::run();
}

fn main() {
    run();
}

#[test]
fn test_main() {
    run();
}
