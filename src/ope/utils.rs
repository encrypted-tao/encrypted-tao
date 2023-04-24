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
 extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
 extern crate rand;
 extern crate rand_distr;
 extern crate hmac;
 extern crate aes;
 extern crate sha2;



 use hmac::{Hmac, Mac};
 use sha2::Sha256;
 use aes::Aes256;
 use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit};
 use generic_array::{GenericArray, arr, ArrayLength};
 use crypto::symmetriccipher::{SynchronousStreamCipher, Encryptor};
 use crypto::aes::{KeySize, ctr};

 pub fn aes_init(result: &mut [u8]) ->  Box<dyn SynchronousStreamCipher + 'static> {

    let aes_cipher =  ctr(KeySize::KeySize256, result, &[0;16]);
    //Aes256::new(&mut GenericArray::from_slice(key));

    return aes_cipher;
 }

