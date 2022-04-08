use hmac::{Hmac, Mac};
use sha2::Sha512;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hpe_eproc_20220407(name: &str) -> String {
    const SALT: &[u8] = b"60e0f7df-92da-4d98-ba56-48f44673387e"; // pick any GUID

    let mut mac =
        Hmac::<Sha512>::new_from_slice(SALT).expect("Should be able to take key of any size");
    mac.update(name.to_lowercase().trim().as_bytes());
    format!("{:x}", mac.finalize().into_bytes())
}

#[test]
fn various_emails() {
    let tests = [
        "tooken@pickuplanet.com",
        "scottlee@comcast.net",
        "subir@yahoo.com",
        "denism@optonline.net",
        "bogjobber@att.net",
        "marnanel@me.com",
        "dmath@yahoo.ca",
        "intlprog@optonline.net",
        "steve@yahoo.com",
        "cparis@sbcglobal.net",
        "sriha@yahoo.ca",
        "galbra@me.com",
        "demmel@me.com",
    ];

    for email in &tests {
        let t = hpe_eproc_20220407(email);
        println!("{}", t);
    }
}
