/*
 * File: crypto.rs
 *      Query encryption/ Result decryption
 *
 * Rust Crypto Crate Source:
 *      https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs
 *      https://github.com/DaGenix/rust-crypto/blob/master/src/symmetriccipher.rs#L58
 *      https://github.com/DaGenix/rust-crypto/blob/master/src/symmetriccipher.rs#L65
 *      https://github.com/project-oak/tink-rust/blob/main/docs/RUST-HOWTO.md#storing-and-loading-existing-keysets
 */
extern crate aes;
extern crate crypto;
extern crate hmac;
extern crate sha2;
extern crate tink_core;
extern crate tink_daead;
extern crate tink_proto;

use crate::ope::ope::ope::Range;
use crate::ope::ope::ope::OPE;
use crate::query::results::DBRow;
use base64::{engine::general_purpose, Engine as _};
use crypto::aes::{ctr, ecb_decryptor, ecb_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::symmetriccipher::{Decryptor, Encryptor, SynchronousStreamCipher};
use tink_core::DeterministicAead;
use tink_proto::KeyTemplate;

use crate::query::query::{Query, TaoArgs, TaoOp};
use crypto::buffer::{
    BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer,
};

pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 - 1;
pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

pub struct CryptKeys {
    ope_key: String,
    aes_key: String,
}

impl CryptKeys {
    fn new(env_path: &String) -> Self {
        dotenv::from_path(env_path).ok();
        let ope_key = dotenv::var("OPE_KEY").unwrap();
        let aes_key = dotenv::var("AES_KEY").unwrap();
        CryptKeys { ope_key, aes_key }
    }
}

pub struct Cipher {
    pub aes_cipher: Box<dyn DeterministicAead>,
}
impl Cipher {
    fn mydefault() -> Cipher {
        tink_daead::init();
        let aeskey =
            tink_core::keyset::Handle::new(&tink_daead::aes_siv_key_template())
                .unwrap();
        let aescipher = tink_daead::new(&aeskey).unwrap();

        Cipher {
            aes_cipher: aescipher,
        }
    }
}
unsafe impl Sync for Cipher {}
unsafe impl Send for Cipher {}
//static cipher: Cipher =  Cipher::mydefault();

//static key_template: KeyTemplate = tink_daead::aes_siv_key_template();
pub struct TaoCrypto {
    keys: CryptKeys,
    cipher: Cipher,
}

impl TaoCrypto {
    pub fn new(env_path: &String) -> Self {
        let keys = CryptKeys::new(env_path);

        tink_daead::init();
        let mycipher: Cipher = Cipher::mydefault();
        TaoCrypto {
            keys,
            cipher: mycipher,
        } //, cipher }
    }

    pub fn encrypt_query(&self, query: Query) -> Query {
        let op = query.op;
        let args = match query.args {
            TaoArgs::AssocAddArgs {
                id1,
                atype,
                id2,
                time,
                data,
            } => TaoArgs::AssocAddArgs {
                id1: self.encrypt_string(id1),
                atype: self.encrypt_string(atype),
                id2: self.encrypt_string(id2),
                time: self.encrypt_ope(time),
                data: self.encrypt_string(data),
            },
            TaoArgs::AssocGetArgs { id, atype, idset } => {
                TaoArgs::AssocGetArgs {
                    id: self.encrypt_string(id),
                    atype: self.encrypt_string(atype),
                    idset: self.encrypt_idset(idset),
                }
            }
            TaoArgs::AssocRangeGetArgs {
                id,
                atype,
                idset,
                tstart,
                tend,
            } => TaoArgs::AssocRangeGetArgs {
                id: self.encrypt_string(id),
                atype: self.encrypt_string(atype),
                idset: self.encrypt_idset(idset),
                tstart: self.encrypt_ope(tstart),
                tend: self.encrypt_ope(tend),
            },
            TaoArgs::AssocCountArgs { id, atype } => TaoArgs::AssocCountArgs {
                id: self.encrypt_string(id),
                atype: self.encrypt_string(atype),
            },
            TaoArgs::AssocRangeArgs {
                id,
                atype,
                tstart,
                tend,
                lim,
            } => TaoArgs::AssocRangeArgs {
                id: self.encrypt_string(id),
                atype: self.encrypt_string(atype),
                tstart: self.encrypt_ope(tstart),
                tend: self.encrypt_ope(tend),
                lim: lim,
            },
            TaoArgs::ObjGetArgs { id } => TaoArgs::ObjGetArgs {
                id: self.encrypt_string(id),
            },
            TaoArgs::ObjAddArgs { id, otype, data } => TaoArgs::ObjAddArgs {
                id: self.encrypt_string(id),
                otype: self.encrypt_string(otype),
                data: self.encrypt_string(data),
            },
        };

        Query { op: op, args: args }
    }

    pub fn encrypt_ope(&self, data: i64) -> i64 {
        let mut ope = OPE {
            key: "ope-testing-key".to_string(),
            in_range: Range {
                start: 1,
                end: DEFAULT_INPUT_RANGE_END,
            },
            out_range: Range {
                start: 1,
                end: DEFAULT_OUTPUT_RANGE_END,
            },
        };
        let encrypt = ope.encrypt(data.try_into().unwrap());

        return encrypt.try_into().unwrap();
    }

    pub fn encrypt_string(&self, data: String) -> String {
        let encrypted = self
            .cipher
            .aes_cipher
            .encrypt_deterministically(&data.into_bytes(), b"")
            .unwrap();
        let encoded = general_purpose::STANDARD_NO_PAD.encode(encrypted);

        return encoded;
    }

    pub fn encrypt_idset(&self, data: Vec<String>) -> Vec<String> {
        let mut encrypt = data.clone();

        for i in 0..data.len() {
            encrypt[i] = self.encrypt_string(data[i].clone());
        }
        return encrypt;
    }

    pub fn decrypt_result(&mut self, row: DBRow) -> DBRow {
        match row {
            DBRow::AssocRow {
                id1,
                atype,
                id2,
                t,
                data,
            } => DBRow::AssocRow {
                id1: self.decrypt_string(id1),
                atype: self.decrypt_string(atype),
                id2: self.decrypt_string(id2),
                t: self.decrypt_ope(t),
                data: self.decrypt_string(data),
            },
            DBRow::ObjRow { id, otype, data } => DBRow::ObjRow {
                id: self.decrypt_string(id),
                otype: self.decrypt_string(otype),
                data: self.decrypt_string(data),
            },
            DBRow::Count(n) => DBRow::Count(n),
            DBRow::NoRes(_) => DBRow::NoRes(true),
        }
    }

    pub fn decrypt_ope(&mut self, data: i64) -> i64 {
        let mut ope = OPE {
            key: "ope-testing-key".to_string(),
            in_range: Range {
                start: 1,
                end: DEFAULT_INPUT_RANGE_END,
            },
            out_range: Range {
                start: 1,
                end: DEFAULT_OUTPUT_RANGE_END,
            },
        };
        let decrypt = ope.decrypt(data.try_into().unwrap());

        return decrypt.try_into().unwrap();
    }

    pub fn decrypt_string(&mut self, data: String) -> String {
        let decoded = match general_purpose::STANDARD_NO_PAD.decode(data) {
            Ok(v) => v,
            Err(e) => panic!("Decode Fail: {}", e),
        };
        let decrypted = self
            .cipher
            .aes_cipher
            .decrypt_deterministically(&decoded, b"")
            .unwrap();
        let plaintext: String = String::from_utf8_lossy(&decrypted).to_string();
        return plaintext;
    }
}

/*
 * Query crypto test
 * run `via cargo test`
 */
#[cfg(test)]
mod tests {
    use crate::ope::ope::ope::Range;
    use crate::ope::ope::ope::OPE;
    use crate::query::crypto::TaoCrypto;

    #[test]
    fn test_encrypt_idset() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let idset = vec!["78".to_string()];
        let encrypt = taocrypt.encrypt_idset(idset);
        println!("Encrypted ID set {:?}\n", encrypt);
        //assert_eq!(vec![23, 18, 20, 25], encrypt);
    }

    #[test]
    fn test_enccrypt_decrypt_string() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let encrypt = taocrypt.encrypt_string("testing".to_string());
        let decrypt = taocrypt.decrypt_string(encrypt.clone());
        println!("Encrypted String {:#?}\n", encrypt);
        println!("Decrypted String {:#?}\n", decrypt.as_bytes());
        assert_eq!(decrypt, "testing".to_string());
    }
}
