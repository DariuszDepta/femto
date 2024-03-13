use cosmwasm_std::testing::MockApi;
use cw_multi_test::App;
use crate::contracts::counter;

pub fn run() {
  store_code_should_work();
}

fn store_code_should_work() {
  // prepare the test application
  let mut app = App::default();

  // store the contract's code
  let code_id = app.store_code(counter::contract());
  assert_eq!(1, code_id);

  // retrieve the metadata (info) describing the stored contract code
  let code_info_response = app.wrap().query_wasm_code_info(code_id).unwrap();

  // check the address of the creator, it should be the default value
  assert_eq!(
    MockApi::default().addr_make("creator").as_str(),
    code_info_response.creator.as_str()
  );
}
