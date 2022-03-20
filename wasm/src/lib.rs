extern crate core;

use json5::Error;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct SyntaxError {
    row: u32,
    column: u32,
    message: Vec<u8>,
}

impl From<Error> for SyntaxError {
    fn from(err: Error) -> SyntaxError {
        let message = err.to_string();
        let vec: Vec<&str> = message.split("\n").collect();
        let first = vec[0];
        let i = first.find("-->").unwrap();
        let j = first.find(":").unwrap();
        let row = first[(i + 3)..j].trim_start().parse::<u32>().unwrap() - 1;
        let column = first[(j + 1)..].trim_end().parse::<u32>().unwrap() - 1;
        let remaining: Vec<String> = vec.iter().skip(5).map(|&it| it.to_string()).collect();
        let message = remaining.join("\n");
        let message = message.trim().trim_start_matches("= ");
        let message = message.as_bytes().to_vec();
        SyntaxError {
            row,
            column,
            message,
        }
    }
}
impl SyntaxError {
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.message.len() + 8);
        vec.append(&mut self.row.to_le_bytes().to_vec());
        vec.append(&mut self.column.to_le_bytes().to_vec());
        vec.append(&mut self.message.to_vec());
        vec
    }
}

fn value(data: &[u8]) -> Result<Value, SyntaxError> {
    Ok(json5::from_str::<Value>(unsafe {
        std::str::from_utf8_unchecked(data)
    })?)
}

#[wasm_bindgen]
pub fn minify(data: &[u8]) -> Vec<u8> {
    let data = match value(data) {
        Ok(value) => value,
        Err(err) => return err.to_vec(),
    };
    // let mut vec = Vec::with_capacity(4096);
    // let formatter = serde_json::Serializer::with_formatter(&mut vec, )
    // data.serialize()
    match serde_json::to_vec(&data) {
        Ok(vec) => vec,
        Err(err) => err.to_string().as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn indent(data: &[u8]) -> Vec<u8> {
    let data = match value(data) {
        Ok(value) => value,
        Err(err) => return err.to_vec(),
    };
    return match serde_json::to_string_pretty(&data) {
        Ok(str) => str.as_bytes().to_vec(),
        Err(err) => err.to_string().as_bytes().to_vec(),
    };
}

#[wasm_bindgen]
pub fn lint(data: &[u8]) -> Vec<u8> {
    match value(data) {
        Ok(_) => Vec::new(),
        Err(err) => return err.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use crate::indent;
    use crate::lint;
    use crate::minify;

    #[test]
    fn test_minify() {
        let bytes =
            minify(r#"["double-quoted",'single-quoted',.5,5.,0,1,-1.e-4,-.1e2]"#.as_bytes());
        let result = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(
            result,
            r#"["double-quoted","single-quoted",0.5,5.0,0,1,-0.0001,-10.0]"#
        )
    }

    #[test]
    fn test_indent() {
        let bytes =
            indent(r#"["double-quoted",'single-quoted',.5,5.,0,1,-1.e-4,-.1e2]"#.as_bytes());
        let result = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(
            result,
            r#"
[
  "double-quoted",
  "single-quoted",
  0.5,
  5.0,
  0,
  1,
  -0.0001,
  -10.0
]
            "#
            .trim()
        )
    }

    #[test]
    fn test_minify_with_syntax_error() {
        let bytes = minify(r#"{]"#.as_bytes());
        let row = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let column = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(row, 0);
        assert_eq!(column, 1);
        let result = std::str::from_utf8(&bytes[8..]).unwrap();
        assert_eq!(result, r#"expected identifier or string"#)
    }

    #[test]
    fn test_indent_with_syntax_error() {
        let bytes = indent(r#"{123}"#.as_bytes());
        let row = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let column = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(row, 0);
        assert_eq!(column, 1);
        let result = std::str::from_utf8(&bytes[8..]).unwrap();
        assert_eq!(result, r#"expected identifier or string"#)
    }

    #[test]
    fn test_lint_with_syntax_error() {
        let bytes = lint(r#"{a::}"#.as_bytes());
        let row = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let column = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(row, 0);
        assert_eq!(column, 3);
        let result = std::str::from_utf8(&bytes[8..]).unwrap();
        assert_eq!(
            result,
            r#"expected array, boolean, null, number, object, or string"#
        )
    }

    #[test]
    fn test_link_huge_number() {
        // let bytes =
        //     lint(r#"["double-quoted",'single-quoted',1234567890123456789123456789]"#.as_bytes());
        // let row = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        // let column = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        // assert_eq!(row, 0);
        // assert_eq!(column, 33);
        // let result = std::str::from_utf8(&bytes[8..]).unwrap();
        // assert_eq!(result, r#"number too large"#)
    }
}
