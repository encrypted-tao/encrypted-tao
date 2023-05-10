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

use crypto::aes::{ctr, ecb_decryptor, ecb_encryptor, KeySize};
use crypto::symmetriccipher::{SynchronousStreamCipher, Decryptor, Encryptor};
use crypto::blockmodes::PkcsPadding;
use tink_core::DeterministicAead;
use tink_proto::KeyTemplate;
use crate::ope::ope::ope::Range;
use crate::ope::ope::ope::OPE;
use crate::query::results::DBRow;

use crypto::buffer::{ RefReadBuffer, RefWriteBuffer, ReadBuffer, WriteBuffer, BufferResult }; 
use crate::query::query::{Query, TaoArgs, TaoOp};

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
   pub  aes_cipher:  Box<dyn DeterministicAead>
}
impl Cipher {
    fn mydefault() -> Cipher {
        tink_daead::init();
         let aeskey = tink_core::keyset::Handle::new(&tink_daead::aes_siv_key_template()).unwrap();
         let aescipher = tink_daead::new(&aeskey).unwrap();

         Cipher { aes_cipher: aescipher }
    }
}
unsafe impl Sync for Cipher {} 
unsafe impl Send for Cipher {}
//static cipher: Cipher =  Cipher::mydefault();


//static key_template: KeyTemplate = tink_daead::aes_siv_key_template();
pub struct TaoCrypto {
    keys: CryptKeys,
    cipher: Cipher,
    //cipher: Box<dyn DeterministicAead>,
    //    ope: OPE,
    //   aes: Box<dyn SynchronousStreamCipher + 'static>
}

impl TaoCrypto {
    pub fn new(env_path: &String) -> Self {
        let keys = CryptKeys::new(env_path);
        
        tink_daead::init();
        let mycipher: Cipher =  Cipher::mydefault();
        //let aeskey = tink_core::keyset::Handle::new(&tink_daead::aes_siv_key_template()).unwrap();
        //let cipher = tink_daead::new(&aeskey).unwrap();
        TaoCrypto { keys, cipher: mycipher } //, cipher }
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
                id1: self.encrypt_int(id1),
                atype: self.encrypt_string(atype),
                id2: self.encrypt_int(id2),
                time: self.encrypt_ope(time),
                data: self.encrypt_string(data),
            },
            TaoArgs::AssocGetArgs { id, atype, idset } => {
                TaoArgs::AssocGetArgs {
                    id: self.encrypt_int(id),
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
                id: self.encrypt_int(id),
                atype: self.encrypt_string(atype),
                idset: self.encrypt_idset(idset),
                tstart: self.encrypt_ope(tstart),
                tend: self.encrypt_ope(tend),
            },
            TaoArgs::AssocCountArgs { id, atype } => TaoArgs::AssocCountArgs {
                id: self.encrypt_int(id),
                atype: self.encrypt_string(atype),
            },
            TaoArgs::AssocRangeArgs {
                id,
                atype,
                tstart,
                tend,
                lim,
            } => TaoArgs::AssocRangeArgs {
                id: self.encrypt_int(id),
                atype: self.encrypt_string(atype),
                tstart: self.encrypt_ope(tstart),
                tend: self.encrypt_ope(tend),
                lim: lim,
            },
            TaoArgs::ObjGetArgs { id } => TaoArgs::ObjGetArgs {
                id: self.encrypt_int(id),
            },
            TaoArgs::ObjAddArgs { id, otype, data } => TaoArgs::ObjAddArgs {
                id: self.encrypt_int(id),
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

    pub fn encrypt_int(&self, data: i64) -> i64 {
        let data_bytes = [data.to_le_bytes()[0]];

        let res = self.cipher.aes_cipher.encrypt_deterministically(&data_bytes, b"test").unwrap();
        println!("INTEGER ENCRYPTION for data {:?} result {:?}", data_bytes, res);
        return res[0].into();
 
    }

    pub fn encrypt_string(&self, data: String) -> String {
       
        let data_bytes =  data.as_bytes();
        let data_test: String = data_bytes.iter().map(ToString::to_string).collect();
        let res = self.cipher.aes_cipher.encrypt_deterministically(&data_bytes, b"test").unwrap();
    
        
        let data_string: String = res.iter().map(ToString::to_string).collect();


        let bytes =  data_string.as_bytes();

        println!("RES {:?} VS STRING {:?}", res, bytes); // shows difference between return type and encryption output
        
        let res1 = self.cipher.aes_cipher.decrypt_deterministically(&res, b"test").unwrap();

        let data_string1 =  String::from_utf8_lossy(&res1);
        //res1.iter().map(ToString::to_string).collect();
        println!("decrypted test {} + {}", data_test, data_string1);
        assert_eq!(&data[..], data_string1);
        println!("decrypted test {} + {}", data_test, data_string1);
      
      
        return data_string;
        
    }

    pub fn encrypt_idset(&self, data: Vec<i64>) -> Vec<i64> {
        let mut encrypt = data.clone();

        for i in 0..data.len() {
            encrypt[i] = self.encrypt_int(data[i]);
        }
        return encrypt;
    }
    pub fn decrypt_result(&mut self, row:  DBRow) ->  DBRow {
        match row {
            DBRow::AssocRow {id1, atype, id2, t, data} => {
                DBRow::AssocRow {
                    id1: self.decrypt_int(id1),
                    atype: self.decrypt_string(atype),
                    id2: self.decrypt_int(id2),
                    t: self.decrypt_ope(t),
                    data: self.decrypt_string(data),
                }
            },
            DBRow::ObjRow {id, otype, data} => {
                DBRow::ObjRow {
                    id: self.decrypt_int(id),
                    otype: self.decrypt_string(otype),
                    data: self.decrypt_string(data),
                }
            },
            DBRow::Count(_) | DBRow::NoRes(_) => todo!()
        }
    }
    pub fn decrypt_int(&mut self, data: i64) -> i64 {
    
        let data_bytes =  [data.to_le_bytes()[0]];
        let res = self.cipher.aes_cipher.decrypt_deterministically(&data_bytes, b"test").unwrap();
    
        return res[0].into();
    
    
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

        let data_bytes =  data.as_bytes();
        let res = self.cipher.aes_cipher.decrypt_deterministically(&data_bytes, b"test").unwrap();
    
        let data_string: String = res.iter().map(ToString::to_string).collect();

        return data_string;

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
    fn test_encrypt_int() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let res = taocrypt.encrypt_int(8);
        println!("testing encryption for int {}\n", res);
        //assert_eq!(res, 56);
    }

    #[test]
    fn test_encrypt_idset() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let encrypt = taocrypt.encrypt_idset(vec![78, 2, 4, 99]);
        println!("Encrypted ID set {:?}\n", encrypt);
        //assert_eq!(vec![23, 18, 20, 25], encrypt);
    }

    #[test]
    fn test_encrypt_string() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let encrypt = taocrypt.encrypt_string("testing".to_string());
        //let test = "116101115116105110103".to_string();

        //assert_eq!(encrypt, test);
    }
    #[test]
    fn test_decrypt_int() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let res = taocrypt.encrypt_int(8);
        println!("Encrypted {}, Decrypted {}\n",taocrypt.encrypt_int(8), taocrypt.decrypt_int(res));
        //assert_eq!(taocrypt.decrypt_int(res), 8);


    }
    #[test]
    fn test_decrypt_string() 
    {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let encrypt = taocrypt.encrypt_string("testing".to_string());

        println!("Encrypted String {:?}\n", encrypt.as_bytes());
        println!("Decrypted String {:?}\n", taocrypt.decrypt_string(encrypt).as_bytes());
       // assert_eq!(taocrypt.decrypt_string(encrypt), "testing".to_string());

    }
}
