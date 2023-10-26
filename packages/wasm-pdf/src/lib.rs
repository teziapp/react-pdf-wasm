use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
extern crate wasm_pdf_gen;

use wasm_pdf_gen::pdf::create;
use wasm_pdf_gen::pdf::json::JsDocument;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = generatePDF)]
    fn generate_file(s: &[u8]);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
    #[wasm_bindgen(js_name = jsonOut)]
    pub fn json_out(data: &JsValue);
}

// #[wasm_bindgen]
// pub fn create_pdf(json: &JsValue) -> Result<JsValue, JsValue> {
//     // output panics to console.error
//     console_error_panic_hook::set_once();
//     let js_doc = get_js_doc(&json)?;
//     let bytes = match create(&js_doc) {
//         Ok(b) => b,
//         Err(s) => return Err(JsValue::from_str(s)),
//     };
//     let array = Uint8Array::from(bytes.as_ref());

//     let blob = Blob::new_with_u8_array_sequence_and_options(
//         &array,
//         BlobPropertyBag::new().type_("application/pdf"),
//     )
//     .map_err(|e| JsValue::from_str(&e.to_string()))?;

//     Ok(JsValue::from(blob))
// }

#[wasm_bindgen]
pub fn run(json: &JsValue) -> Result<(), JsValue> {
    // output panics to console.error
    console_error_panic_hook::set_once();
    let js_doc = get_js_doc(&json)?;
    let bytes = match create(&js_doc) {
        Ok(b) => b,
        Err(s) => return Err(JsValue::from_str(s)),
    };
    generate_file(&bytes);
    Ok(())
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
