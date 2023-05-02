/*
 * File: crypto.rs
 *      Query encryption/decryption
 */
extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate hmac;
extern crate aes;
extern crate sha2;

//use crate::ope::ope::hgd::PRNG;
use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::aes::{KeySize, ctr};

pub fn retrieve_key() -> String {

    return "my-tao-testing-key".to_string();

}
pub fn encrypt_ope(data: u64) -> u64 {

    return data;
}

pub fn encrypt_string(data: String) -> String {

    return data;
}

pub fn encrypt_int(data: i32) -> i32 {

    let key = retrieve_key();
    let mut aes_cipher =  ctr(KeySize::KeySize256, &key.into_bytes(), &[b'\x00';16]); // change IV?

    let data_string = data.to_string();
    let data_bytes = data_string.into_bytes();

    let mut res = 0;
    aes_cipher.process(&data_bytes, &mut data_bytes.clone());

    return data_bytes[0].into();


}

pub fn encrypt_idset(data: Vec<i32>) -> Vec<i32> {

    return data;

}
/*
 * Query crypto test
 * run `via cargo test`
 */
#[cfg(test)]
mod tests {

    use crate::query::crypto::{encrypt_int};

    #[test]
    fn test_encrypt_int() {
        let res = encrypt_int(8);
        assert_eq!(res, 56);
    }
}