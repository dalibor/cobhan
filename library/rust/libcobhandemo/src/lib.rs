use std::os::raw::c_char;
use std::{thread, time};
use serde_json::{Value};
use std::collections::HashMap;
use rand::Rng;
use rand::RngCore;

#[no_mangle]
pub unsafe extern "C" fn sleepTest(seconds: i32) {
    thread::sleep(time::Duration::from_secs(seconds as u64))
}

#[no_mangle]
pub unsafe extern "C" fn addInt32(x: i32, y: i32) -> i32 {
    x.saturating_add(y)
}

#[no_mangle]
pub unsafe extern "C" fn addInt64(x: i64, y: i64) -> i64 {
    x.saturating_add(y)
}

#[no_mangle]
pub unsafe extern "C" fn addDouble(x: f64, y: f64) -> f64 {
    x + y
}

#[no_mangle]
pub unsafe extern "C" fn toUpper(input: *const c_char, output: *mut c_char) -> i32 {
    let input_str;
    match cobhan::cbuffer_to_string(input) {
        Ok(str) => input_str = str,
        Err(e) => return e
    }

    let output_str = input_str.to_uppercase();

    cobhan::string_to_cbuffer(&output_str, output)
}

#[no_mangle]
pub unsafe extern "C" fn filterJson(input: *const c_char, disallowed_value: *const c_char, output: *mut c_char) -> i32 {
    let mut json;
    match cobhan::cbuffer_to_hashmap_json(input) {
        Ok(input_json) => json = input_json,
        Err(e) => return e
    }

    let disallowed_value_str;
    match cobhan::cbuffer_to_string(disallowed_value) {
        Ok(disallow) => disallowed_value_str = disallow,
        Err(e) => return e
    }

    filter_json(&mut json, &disallowed_value_str);

    cobhan::hashmap_json_to_cbuffer(&json, output)
}

// Example of a safe function
pub fn filter_json(json: &mut HashMap<String, Value>, disallowed: &str) {
    json.retain(|_key, value| {
        return match value.as_str() {
            None => true,
            v => !v.unwrap().contains(&disallowed)
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn base64Encode(input: *const c_char, output: *mut c_char) -> i32 {
    let bytes;
    match cobhan::cbuffer_to_vector(input) {
        Ok(b) => bytes = b,
        Err(e) => return e
    }

    let b64str = base64::encode(bytes);

    cobhan::string_to_cbuffer(&b64str, output)
}

#[no_mangle]
pub unsafe extern "C" fn generateRandom(output: *mut c_char) -> i32 {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(0..134217728);
    let mut bytes: Vec<u8> = vec![0; size];
    rng.fill_bytes(&mut bytes);
    cobhan::bytes_to_cbuffer(&bytes, output)
}
