use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
extern crate wasm_pdf_gen;

use wasm_pdf_gen::pdf::create;
use wasm_pdf_gen::pdf::json::JsDocument;

use js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
    #[wasm_bindgen(js_name = jsonOut)]
    pub fn json_out(data: &JsValue);
}

#[wasm_bindgen]
pub fn create_pdf(json: &JsValue) -> Result<JsValue, JsValue> {
    // output panics to console.error
    console_error_panic_hook::set_once();
    let js_doc = get_js_doc(&json)?;
    let bytes = match create(&js_doc) {
        Ok(b) => b,
        Err(s) => return Err(JsValue::from_str(s)),
    };

    // Convert the Vec<u8> to a Uint8Array
    let uint8_array = Uint8Array::from(bytes.as_slice());

    // Return the Uint8Array as a JsValue
    Ok(uint8_array.into())
}

/// Test Utility: Exports serde objects to json_out function (JS)
#[wasm_bindgen]
pub fn print_document(json: &JsValue) -> Result<(), JsValue> {
    let js_doc = get_js_doc(&json)?;
    let out = JsValue::from_serde(&js_doc).unwrap();
    json_out(&out);
    Ok(())
}

fn get_js_doc(json: &JsValue) -> Result<JsDocument, JsValue> {
    match json.into_serde() {
        Ok(doc) => Ok(doc),
        Err(e) => Err(JsValue::from_str(&format!(
            "Error. Could not parse JSON data. {}",
            e
        ))),
    }
}
