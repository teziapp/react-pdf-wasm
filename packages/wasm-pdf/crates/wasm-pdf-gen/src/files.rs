use crate::pdf::create;
use crate::pdf::json::JsDocument;
use crate::pdf::json::JsParamValue;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;

/// Parse a jpg file to get width and height. Currently only jpg images are supported.
fn read_file(path: &Path, bytes: &mut Vec<u8>) -> Result<(f32, f32), String> {
    let mut input_file = File::open(path).map_err(|err| format!("Error opening jpg: {}", err))?;
    let len = input_file.read_to_end(bytes).unwrap();
    let mut pos: usize = 0;

    if bytes[pos] == 0xFF
        && bytes[pos + 1] == 0xD8
        && bytes[pos + 2] == 0xFF
        && bytes[pos + 3] == 0xE0
    {
        pos += 4;
        if bytes[pos + 2] == b'J'
            && bytes[pos + 3] == b'F'
            && bytes[pos + 4] == b'I'
            && bytes[pos + 5] == b'F'
            && bytes[pos + 6] == 0x00
        {
            let mut block_len: usize = bytes[pos] as usize * 256 + bytes[pos + 1] as usize;
            while pos < len {
                pos += block_len as usize;
                if pos >= len {
                    return Err("EOF.".to_string());
                }
                if bytes[pos] != 0xFF {
                    return Err("Entered another block.".to_string());
                }
                if bytes[pos + 1] == 0xC0 {
                    let height = bytes[pos + 5] as usize * 256 + bytes[pos + 6] as usize;
                    let width = bytes[pos + 7] as usize * 256 + bytes[pos + 8] as usize;
                    return Ok((width as f32, height as f32));
                } else {
                    pos += 2;
                    block_len = bytes[pos] as usize * 256 + bytes[pos + 1] as usize;
                }
            }
        }
    }
    Err("Invalid jpg format.".to_string())
}

fn add_image_data_to_js(src: &str, base_path: &Path) -> Result<(String, f32, f32), String> {
    let image_path = base_path.join(&src);
    let mut img_buffer: Vec<u8> = Vec::new();
    let (width, height) = read_file(&image_path, &mut img_buffer)
        .map_err(|err| format!("Error reading file: {}", err))?;
    let image_data = base64::encode(&img_buffer);
    Ok((image_data, width, height))
}

/// Process JSON and save PDF.
pub fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let input_file =
        File::open(input_fname).map_err(|err| format!("Error opening input: {}", err))?;
    let reader = BufReader::new(input_file);
    let base_path = Path::new(input_fname).parent().unwrap();
    let mut js_doc: JsDocument =
        serde_json::from_reader(reader).expect("Error parsing JSON document.");
    for content in &js_doc.contents {
        match content.obj_type.to_lowercase().as_str() {
            "table" => {
                if let Some(rows) = content.params.get("rows") {
                    if let JsParamValue::Children(rows) = rows {
                        for row in rows {
                            if let Some(cells) = row.params.get("cells") {
                                if let JsParamValue::Children(cells) = cells {
                                    for cell in cells {
                                        if let Some(cell_contents) = cell.params.get("contents") {
                                            if let JsParamValue::Children(contents) = cell_contents
                                            {
                                                for cell_content in contents {
                                                    if let "image" = cell_content
                                                        .obj_type
                                                        .to_lowercase()
                                                        .as_str()
                                                    {
                                                        if let Some(src) =
                                                            cell_content.params.get("src")
                                                        {
                                                            if let JsParamValue::Text(src) = src {
                                                                if !js_doc
                                                                    .image_data
                                                                    .contains_key(src)
                                                                {
                                                                    let (img_data, width, height) =
                                                                        add_image_data_to_js(
                                                                            src, base_path,
                                                                        )?;
                                                                    js_doc.image_data.insert(
                                                                        src.to_owned(),
                                                                        img_data,
                                                                    );
                                                                    js_doc.image_heights.insert(
                                                                        src.to_owned(),
                                                                        height as f32,
                                                                    );
                                                                    js_doc.image_widths.insert(
                                                                        src.to_owned(),
                                                                        width as f32,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            "image" => {
                if let Some(src) = content.params.get("src") {
                    if let JsParamValue::Text(src) = src {
                        if !js_doc.image_data.contains_key(src) {
                            let (img_data, width, height) = add_image_data_to_js(src, base_path)?;
                            js_doc.image_data.insert(src.to_owned(), img_data);
                            js_doc.image_heights.insert(src.to_owned(), height as f32);
                            js_doc.image_widths.insert(src.to_owned(), width as f32);
                        }
                    }
                }
            }
            _ => (),
        }
    }

    let bytes = match create(&js_doc) {
        Ok(b) => b,
        Err(s) => s.into(),
    };
    let mut output_file = File::create(output_fname)
        .map_err(|err| format!("Error opening output '{}': {}", output_fname, err))?;
    output_file
        .write_all(&bytes)
        .map_err(|err| format!("Write error: {}", err))
}
