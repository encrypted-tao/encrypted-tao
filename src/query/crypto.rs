/*
 * File: crypto.rs
 *      Query encryption/decryption
 */
extern crate aes;
extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate hmac;
extern crate sha2;

use crypto::aes::{ctr, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;

use crate::ope::ope::ope::Range;
use crate::ope::ope::ope::OPE;

use crate::query::query::{Query, TaoArgs, TaoOp};

pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 - 1;
pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

pub struct CryptKeys {
    ope_key: String,
    aes_key: String,
}

impl CryptKeys {
    pub fn new(env_path: &String) -> Self {
        dotenv::from_path(env_path).ok();
        let ope_key = dotenv::var("OPE_KEY").unwrap();
        let aes_key = dotenv::var("AES_KEY").unwrap();
        CryptKeys { ope_key, aes_key }
    }
}

pub struct TaoCrypto {
    keys: CryptKeys,
    //    ope: OPE,
    //   aes: Box<dyn SynchronousStreamCipher + 'static>
}

impl TaoCrypto {
    pub fn new(env_path: &String) -> Self {
        let keys = CryptKeys::new(env_path);
        TaoCrypto { keys }
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
        let data_string = data.to_string();
        let data_bytes = data_string.into_bytes();

        let ak = "my-tao-testing-key".to_string();
        let mut aes =
            ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00'; 16]);
        aes.process(&data_bytes, &mut data_bytes.clone());

        return data_bytes[0].into();
    }

    pub fn encrypt_string(&self, data: String) -> String {
        let ak = "my-tao-testing-key".to_string();
        let mut aes =
            ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00'; 16]);
        let data_bytes = data.into_bytes();
        aes.process(&data_bytes, &mut data_bytes.clone());
        let data_string: String =
            data_bytes.iter().map(ToString::to_string).collect();

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
        }
    }
    pub fn decrypt_int(&mut self, data: i64) -> i64 {

    }
    pub fn decrypt_ope(&mut self, data: i64) -> i64 {

    }
    pub fn decrypt_string(&mut self, data: String) -> String {
        
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
        assert_eq!(res, 56);
    }

    #[test]
    fn test_encrypt_idset() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let encrypt = taocrypt.encrypt_idset(vec![78, 2, 4, 99]);
        assert_eq!(vec![55, 50, 52, 57], encrypt);
    }

    #[test]
    fn test_encrypt_string() {
        let mut taocrypt = TaoCrypto::new(&"./.env".to_string());
        let encrypt = taocrypt.encrypt_string("testing".to_string());
        let test = "116101115116105110103".to_string();

        assert_eq!(encrypt, test);
    }
}
