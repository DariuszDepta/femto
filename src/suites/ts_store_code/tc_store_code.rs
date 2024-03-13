//! # Test cases for `App::store_code`

use crate::contracts::counter;
use cw_multi_test::App;

pub fn run() {
    store_code_should_work();
}

fn store_code_should_work() {
    // prepare the test application
    let mut app = App::default();

    // store the contract's code
    let code_id = app.store_code(counter::contract());
    assert_eq!(1, code_id);
}
