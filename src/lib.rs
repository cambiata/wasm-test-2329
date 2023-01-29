use wasm_bindgen::prelude::*;
use web_sys;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn test() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("h1")?;
    val.set_inner_html("Test");
    body.append_child(&val)?;
    Ok(())
    
}

