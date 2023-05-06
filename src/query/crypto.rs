/*
 * File: crypto.rs
 *      Query encryption/decryption
 */
extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate hmac;
extern crate aes;
extern crate sha2;

use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::aes::{KeySize, ctr};

use crate::ope::ope::ope::OPE;
use crate::ope::ope::ope::Range;

use crate::query::query::{TaoArgs};

pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 -1;
pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

pub struct CryptKeys {
    ope_key: String,
    aes_key: String,
}

impl CryptKeys {
    pub fn new(env_path: String) -> Self {
        dotenv::from_path(env_path).ok();
        let ope_key  = dotenv::var("OPE_KEY").unwrap();
        let aes_key = dotenv::var("AES_KEY").unwrap();
        CryptKeys { ope_key, aes_key }
    }
}

pub struct TaoCrypto {
    ope: OPE,
    aes: Box<dyn SynchronousStreamCipher + 'static>
}

impl TaoCrypto {
    pub fn new(env_path: String) -> Self {
        let keys = CryptKeys::new(env_path);
        let ope = OPE { key: "ope-testing-key".to_string(), 
                        in_range: Range { start: 1, end: DEFAULT_INPUT_RANGE_END }, 
                        out_range: Range { start: 1, end: DEFAULT_OUTPUT_RANGE_END } };
        let ak = "my-tao-testing-key".to_string();
        let aes = ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00';16]);
        TaoCrypto { ope, aes }
    }

    pub fn encrypt_args(&mut self, args: TaoArgs) -> TaoArgs {
        match args {
            TaoArgs::AssocAddArgs {id1, atype, id2, time, data} => {
                TaoArgs::AssocAddArgs {
                    id1: self.encrypt_int(id1),
                    atype: self.encrypt_string(atype),
                    id2: self.encrypt_int(id2),
                    time: self.encrypt_ope(time),
                    data: self.encrypt_string(data),
                }
            },
            TaoArgs::AssocGetArgs {id, atype, idset} => {
                TaoArgs::AssocGetArgs {
                    id: self.encrypt_int(id),
                    atype: self.encrypt_string(atype),
                    idset: self.encrypt_idset(idset),
                } 
            },
            TaoArgs::AssocRangeGetArgs {id, atype, idset, tstart, tend} => {
                TaoArgs::AssocRangeGetArgs {
                    id: self.encrypt_int(id),
                    atype: self.encrypt_string(atype),
                    idset: self.encrypt_idset(idset),
                    tstart: self.encrypt_ope(tstart),
                    tend: self.encrypt_ope(tend),
                }
            },
            TaoArgs::AssocCountArgs {id, atype} => {
                TaoArgs::AssocCountArgs {
                    id: self.encrypt_int(id),
                    atype: self.encrypt_string(atype),
                } 
            },
            TaoArgs::AssocRangeArgs {id, atype, tstart, tend, lim} => {
                TaoArgs::AssocRangeArgs {
                    id: self.encrypt_int(id),
                    atype: self.encrypt_string(atype),
                    tstart: self.encrypt_ope(tstart),
                    tend: self.encrypt_ope(tend),
                    lim: lim,
                }
            },
            TaoArgs::ObjGetArgs {id} => {
                TaoArgs::ObjGetArgs { id: self.encrypt_int(id) }
            },
            TaoArgs::ObjAddArgs {id, otype, data} => {
                TaoArgs::ObjAddArgs {
                    id: self.encrypt_int(id),
                    otype: self.encrypt_string(otype),
                    data: self.encrypt_string(data),
                } 
            },
        }
    }

    pub fn encrypt_ope(&mut self, data: i64) -> i64 {
        let encrypt = self.ope.encrypt(data.try_into().unwrap());

        return encrypt.try_into().unwrap();
    }

    pub fn encrypt_int(&mut self, data: i64) -> i64 {
        let data_string = data.to_string();
        let data_bytes = data_string.into_bytes();
        let mut aes = ctr(KeySize::KeySize256, &"my-tao-testing-key".to_string().into_bytes(), &[b'\x00';16]);
        aes.process(&data_bytes, &mut data_bytes.clone());

        return data_bytes[0].into();
    }

    pub fn encrypt_string(&mut self, data: String) -> String {
        let data_bytes = data.into_bytes();
        let mut aes = ctr(KeySize::KeySize256, &"my-tao-testing-key".to_string().into_bytes(), &[b'\x00';16]);
        aes.process(&data_bytes, &mut data_bytes.clone());
        let data_string: String = data_bytes.iter().map(ToString::to_string).collect();

        return data_string;
    }
    
    pub fn encrypt_idset(&mut self, data: Vec<i64>) -> Vec<i64> {
        let mut encrypt = data.clone();

        for i in 0..data.len() {
            encrypt[i] = self.encrypt_int(data[i]);
        }
        return encrypt;
    }
}

/*
 * Query crypto test
 * run `via cargo test`
 */
#[cfg(test)]
mod tests {
    use crate::query::crypto::TaoCrypto;
    use crate::ope::ope::ope::OPE;
    use crate::ope::ope::ope::Range;

    #[test]
    fn test_encrypt_int() {
        let mut taocrypt = TaoCrypto::new("./.env".to_string());
        let res = taocrypt.encrypt_int(8);
        assert_eq!(res, 56);
    }

    #[test]
    fn test_encrypt_idset() {
        let mut taocrypt = TaoCrypto::new("./.env".to_string());
        let encrypt = taocrypt.encrypt_idset(vec![78, 2, 4, 99]);
        assert_eq!(vec![55, 50, 52, 57], encrypt);
    }

    #[test]
    fn test_encrypt_string() {
        let mut taocrypt = TaoCrypto::new("./.env".to_string());
        let encrypt = taocrypt.encrypt_string("testing".to_string());
        let test = "116101115116105110103".to_string();
        
        assert_eq!(encrypt, test);
    }
}
