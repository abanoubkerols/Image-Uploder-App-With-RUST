use base64::{encode , decode};
use web_sys::console::log_1 as log;
use wasm_bindgen::prelude::*;
use image::load_from_memory;
use image::ImageOutputFormat::Png;
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str)-> String {
    log(&"Grayscale scalled".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"grayscale effect applied".into());

    let mut buffer =vec![];
    img.write_to(&mut buffer , Png).unwrap();
    log(&"new img written");

    let encode_img =encode(&buffer);
    let data_url =fromat!("data:image/png;base64,{}" , encode_img);

    data_url
    

}
