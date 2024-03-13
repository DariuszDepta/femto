#[cfg(not(any(feature = "v_0_1_1", feature = "v_1_1_1")))]
use cosmwasm_std::CodeInfoResponse;

pub fn run() {
    retrieving_code_info_should_work();
}

#[cfg(any(feature = "v_0_1_1", feature = "v_1_1_1"))]
fn retrieving_code_info_should_work() {
    // in cosmwasm-std 1.1 retrieving code info was not provided
}

#[cfg(any(feature = "v_0_1_2", feature = "v_0_1_3", feature = "v_0_1_4"))]
fn retrieving_code_info_should_work() {
    assert_eq!("code-creator", code_info_response().creator.as_str());
}

#[cfg(any(
    feature = "v_1_1_2",
    feature = "v_1_1_3",
    feature = "v_1_1_4",
    feature = "v_2"
))]
fn retrieving_code_info_should_work() {
    use cosmwasm_std::testing::MockApi;

    assert_eq!(
        MockApi::default().addr_make("creator").as_str(),
        code_info_response().creator.as_str()
    );
}

#[cfg(not(any(feature = "v_0_1_1", feature = "v_1_1_1")))]
fn code_info_response() -> CodeInfoResponse {
    use crate::contracts::counter;
    use cw_multi_test::App;
    // prepare the test application
    let mut app = App::default();
    // store the contract's code
    let code_id = app.store_code(counter::contract());
    // retrieve the metadata (info) describing the stored contract code
    app.wrap().query_wasm_code_info(code_id).unwrap()
}
