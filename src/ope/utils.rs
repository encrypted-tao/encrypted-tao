/*
 * File: utils.rs
 *      Cryptographic helper functions for Rust implementation of
 *      Order Preserving Encryption OPE
 *      Based off of a python implementation
 *      (https://github.com/tonyo/pyope/blob/master/pyope/ope.py)
 *
 *      Ref:
 *          https://faculty.cc.gatech.edu/~aboldyre/papers/bclo.pdf
 *          https://people.csail.mit.edu/nickolai/papers/popa-mope-eprint.pdf
 *          https://arxiv.org/pdf/2009.05679.pdf
 *
 */

extern crate aes;
extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate hmac;
extern crate sha2;

use crate::ope::hgd::PRNG;
use crypto::aes::{ctr, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;

pub fn aes_init(
    result: &mut [u8],
) -> Box<dyn SynchronousStreamCipher + 'static> {
    // sanity check
    assert_eq!(result.len(), 32);

    let aes_cipher = ctr(KeySize::KeySize256, result, &[b'\x00'; 16]);

    return aes_cipher;
}

pub fn generate_tape(prng: &mut PRNG) -> [u32; 96] {
    let mut tape = [b'\x00'; 16];

    prng.cipher.process(&[b'\x00'; 16], &mut tape);

    let bin_tape = convert_bitstring(tape);

    return bin_tape;
}

pub fn generate_bytes(data: &mut [u8; 16]) -> [Vec<u8>; 16] {
    let mut bin: [Vec<u8>; 16] = Default::default();

    for i in 0..16 {
        let tmp: String = data[i].to_string();
        bin[i] = tmp.clone().as_bytes().to_vec();
    }

    return bin;
}
pub fn byte_to_bits(byte: &mut Vec<u8>) -> String {
    return format!("{:b}", byte[0]);
}
pub fn convert_bitstring(mut data: [u8; 16]) -> [u32; 96] {
    let bytes = generate_bytes(&mut data);
    let mut ret = [0; 96];
    let mut index = 0;

    for mut byte in bytes {
        let bit = byte_to_bits(&mut byte);
        for i in bit.chars() {
            ret[index] = i.to_digit(2 /* u32 */).unwrap();
            index += 1;
        }
    }
    return ret;
}
