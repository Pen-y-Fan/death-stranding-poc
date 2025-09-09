#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_works_in_browser() {
    assert_eq!(1 + 1, 2);
}
