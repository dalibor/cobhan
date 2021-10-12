use std::os::raw::c_char;
use std::{thread, time};
use cobhan::{input_string, input_hashmap_json, output_string, output_hashmap_json, input_bytes};

const DEFAULT_INPUT_MAXIMUM: i32 = 4096;

#[no_mangle]
pub unsafe extern "C" fn sleepTest(seconds: i32) {
    thread::sleep(time::Duration::from_secs(seconds as u64));
}

#[no_mangle]
pub unsafe extern "C" fn addInt32(x: i32, y: i32) -> i32 {
    return x.saturating_add(y);
}

#[no_mangle]
pub unsafe extern "C" fn addInt64(x: i64, y: i64) -> i64 {
    return x.saturating_add(y);
}

#[no_mangle]
pub unsafe extern "C" fn addDouble(x: f64, y: f64) -> f64 {
    return x + y;
}

#[no_mangle]
pub unsafe extern "C" fn toUpper(input: *const c_char, input_len: i32, output: *mut c_char, output_cap: i32) -> i32 {
    let input_str;
    match input_string(input, input_len, DEFAULT_INPUT_MAXIMUM) {
        Ok(str) => input_str = str,
        Err(e) => return e
    }

    let output_str = input_str.to_uppercase();

    return output_string(&output_str, output, output_cap);
}

#[no_mangle]
pub unsafe extern "C" fn filterJson(input: *const c_char, input_len: i32, disallowed_value: *const c_char, disallowed_value_len: i32, output: *mut c_char, output_cap: i32) -> i32 {
    let mut json;
    match input_hashmap_json(input, input_len, DEFAULT_INPUT_MAXIMUM) {
        Ok(input_json) => json = input_json,
        Err(e) => return e
    }

    let disallowed_value_str;
    match input_string(disallowed_value, disallowed_value_len, DEFAULT_INPUT_MAXIMUM) {
        Ok(str) => disallowed_value_str = str,
        Err(e) => return e
    }

    json.retain(|_key, value| {
        match value.as_str() {
            None => return true,
            v => return v.unwrap().contains(&disallowed_value_str)
        }
    });

    return output_hashmap_json(&json, output, output_cap);
}

#[no_mangle]
pub unsafe extern "C" fn base64Encode(input: *const c_char, input_len: i32, output: *mut c_char, output_cap: i32) -> i32 {
    let bytes;
    match input_bytes(input, input_len, DEFAULT_INPUT_MAXIMUM) {
        Ok(b) => bytes = b,
        Err(e) => return e
    }

    let b64str = base64::encode(bytes);

    return output_string(&b64str, output, output_cap);
}

