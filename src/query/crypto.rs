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

    
    let key = retrieve_key();
    let mut aes_cipher =  ctr(KeySize::KeySize256, &key.into_bytes(), &[b'\x00';16]); // change IV?

    let data_bytes = data.into_bytes();

    aes_cipher.process(&data_bytes, &mut data_bytes.clone());

    let data_string: String = data_bytes.iter().map(ToString::to_string).collect();
    return data_string;
}

pub fn encrypt_int(data: i32) -> i32 {

    let key = retrieve_key();
    let mut aes_cipher =  ctr(KeySize::KeySize256, &key.into_bytes(), &[b'\x00';16]); // change IV?

    let data_string = data.to_string();
    let data_bytes = data_string.into_bytes();

    aes_cipher.process(&data_bytes, &mut data_bytes.clone());

    return data_bytes[0].into();


}

pub fn encrypt_idset(data: Vec<i32>) -> Vec<i32> {

    let mut encrypt = data.clone();

    for i in 0..data.len() {
        encrypt[i] = encrypt_int(data[i]);
    }
    return encrypt;

}
/*
 * Query crypto test
 * run `via cargo test`
 */
#[cfg(test)]
mod tests {

    use crate::query::crypto::{encrypt_int, encrypt_idset, encrypt_string};

    #[test]
    fn test_encrypt_int() {
        let res = encrypt_int(8);
        assert_eq!(res, 56);
    }

    #[test]
    fn test_encrypt_idset() {
        let encrypt = encrypt_idset(vec![78, 2, 4, 99]);
        assert_eq!(vec![55, 50, 52, 57], encrypt);
    }

    #[test]
    fn test_encrypt_string() {
        let encrypt = encrypt_string("testing".to_string());
        let test = "116101115116105110103".to_string();
        
        assert_eq!(encrypt, test);
    }
}