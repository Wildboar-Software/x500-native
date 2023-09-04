#![deny(clippy::all)]

use napi_derive::napi;
use napi::bindgen_prelude::*;
use stringprep::x520prep;

#[napi]
fn oid_from_str(s: String) -> Option<Vec<u32>> {
    let mut ret: Vec<u32> = Vec::with_capacity(s.len());
    for sub in s.split('.') {
        let i: u32 = sub.parse().ok()?;
        ret.push(i);
    }
    return Some(ret);
}

#[napi]
fn oid_from_bytes(b: Uint8Array) -> Option<Vec<u32>> {
    let len = b.len();
    if len < 1 {
        return None;
    }
    let arc1 = (b[0] / 40) as u32;
    let arc2 = (b[0] % 40) as u32;
    // In pre-allocating, we assume the average OID arc consumes two bytes.
    let mut nodes: Vec<u32> = Vec::with_capacity(len << 1);
    nodes.push(arc1);
    nodes.push(arc2);
    let mut current_node: u32 = 0;
    for byte in b[1..].iter() {
        if (current_node == 0) && (*byte == 0b1000_0000) {
            return None;
        }
        current_node <<= 7;
        current_node += (byte & 0b0111_1111) as u32;
        if (byte & 0b1000_0000) == 0 {
            nodes.push(current_node);
            current_node = 0;
        }
    }
    if current_node > 0 {
        return None;
    }
    Some(nodes)
}

#[napi]
fn prep_string (s: String, case_fold: bool) -> Option<String> {
    match x520prep(&s, case_fold) {
        Ok(p) => Some(s.to_owned()),
        Err(_) => None,
    }
}

// #[napi]
// fn x520_prep_string_compare (a: &str, b: &str, ignore_case: bool) -> bool {
//     // Waiting for the prepstring package to accept PR.
//     let prepped_a = x520prep(a);
//     let prepped_b = x520prep(b, case_fold);
// }

// TODO: Native Teletex transcoding
// TODO: Native Hashmap / Hashset
// TODO: caseIgnoreMatch
// TODO: caseExactMatch
// TODO: numericStringMatch
// TODO: prepString
