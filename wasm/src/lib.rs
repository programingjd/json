extern crate core;

use std::error::Error;
use nu_json::Value;
use serde::Serialize;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn minify(data: &[u8]) -> Vec<u8> {
    let data: Value = match nu_json::from_slice(data) {
        Ok(value) => value,
        Err(err) => return err.to_string().as_bytes().to_vec()
    };
    match serde_json::to_vec(&data) {
        Ok(vec) => vec,
        Err(err) => err.to_string().as_bytes().to_vec()
    }
}

#[wasm_bindgen]
pub fn indent(data: &[u8]) -> Vec<u8> {
    let value: Value = match nu_json::from_slice(data) {
        Ok(value) => value,
        Err(err) => return err.to_string().as_bytes().to_vec()
    };
    return match serde_json::to_string_pretty(&value) {
        Ok(str) => str.as_bytes().to_vec(),
        Err(err) => err.to_string().as_bytes().to_vec()
    }
}
