use ic_cdk::{post_upgrade, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::http::*;

// Http interface
#[query]
async fn http_request(req: RawHttpRequest) -> RawHttpResponse {
    let method = req.method;

    let body = req.body;

    // let number = convert_vec_to_u128(body) * (2 as u128);
    // let first = double_and_convert(body);

    let value: Vec<u8> = vec![49, 50, 52, 56];

    if method == "POST" {
        RawHttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: value[3].to_be_bytes().to_vec(),
            upgrade: None,
        }
    } else {
        let body_data = double_u8_vec(body);

        RawHttpResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: body_data,
            upgrade: None,
        }
    }
}

fn get_real_number(data: Vec<u8>) -> (u128, u8) {
    let mut number: u128 = 0;
    let mut decimal_cnt = 0;
    let mut decimal_exists = false;
    for sub_data in data {
        if sub_data == 46 {
            decimal_exists = true;
            continue;
        } else {
            if decimal_exists == true {
                decimal_cnt += 1;
            }
        }
        let real_data = sub_data - 48 as u8;
        number = number * 10 as u128 + real_data as u128;
    }

    (number, decimal_cnt)
}

fn convert_to_bytes(data: u128, decimal_cnt: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut number = data;
    let mut decimals = decimal_cnt;

    while number != 0 as u128 {
        let sub_data = number % 10 as u128;
        number = number / 10 as u128;
        if sub_data == 0 as u128 {
            decimals -= 1 as u8;
        }
        result.push(sub_data as u8 + 48 as u8);
    }

    let mut len = result.len();
    let mut final_result: Vec<u8> = Vec::new();
    let mut mid_result: Vec<u8> = Vec::new();
    let mut index = 0;
    while len > 0 {
        let data = result.get(len - 1).unwrap();
        mid_result.push(*data);
        len -= 1;
    }

    len = mid_result.len();
    while index < len {
        let data = mid_result.get(index).unwrap();
        final_result.push(*data);
        if index == len - decimal_cnt as usize - 1 as usize && decimal_cnt > 0 {
            final_result.push(46 as u8);
        }
        index += 1;
    }

    final_result
}

// Vec<u8> = [1, 2, 4, 8, '.', 1, 2, 5]
fn double_u8_vec(data: Vec<u8>) -> Vec<u8> {
    let mut number: u128;
    let decimal_cnt: u8;
    (number, decimal_cnt) = get_real_number(data.clone());
    number = number * 2 as u128;
    convert_to_bytes(number.clone(), decimal_cnt.clone())
}

fn double_and_convert(data: Vec<u8>) -> Vec<u8> {
    // Convert the Vec<u8> to a u128
    let number = u128::from_be_bytes(data.try_into().unwrap());

    // Double the number
    let doubled_number = number * 2;

    // Convert the u128 back to Vec<u8>
    doubled_number.to_be_bytes().to_vec()
}

fn vec_to_u128_le(bytes: Vec<u8>) -> Option<u128> {
    if bytes.len() < 16 {
        None // Ensure there are enough bytes to form a u128
    } else {
        let (int_bytes, _) = bytes.split_at(16);
        Some(u128::from_le_bytes(int_bytes.try_into().unwrap()))
    }
}

fn convert_vec_to_u128(vec: Vec<u8>) -> u128 {
    let mut result: u128 = 0;
    for (i, byte) in vec.iter().enumerate() {
        result |= (*byte as u128) << (8 * i);
    }
    result
}

fn convert_u128_to_vec(value: u128) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for i in 0..3 {
        result.push((value >> (8 * i)) as u8);
    }
    result
}