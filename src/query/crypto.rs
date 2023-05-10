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


use crypto::aes::{ctr, ecb_decryptor, ecb_encryptor, KeySize};
use crypto::symmetriccipher::{SynchronousStreamCipher, Decryptor, Encryptor};
use crypto::blockmodes::PkcsPadding;
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
       
        let mut bytes = data_string.into_bytes();
        let mut data_buf = RefReadBuffer::new(&bytes);
        let mut buf = [0; 64];
        let mut out_buf = RefWriteBuffer::new(&mut buf);

        let ak = "my-tao-testing-key".to_string();
        let mut aes =
            ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00'; 16]);
        aes.encrypt(&mut data_buf, &mut out_buf, true);
    
        let mut data_buf = out_buf.take_read_buffer();

        let mut data_bytes = data_buf.take_remaining();

        return bytes[0].into();
    }

    pub fn encrypt_string(&self, data: String) -> String {
        let ak = "my-tao-testing-key".to_string();

        let mut aes =
            ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00'; 16]);

        let mut bytes = data.into_bytes();
        let mut data_buf = RefReadBuffer::new(&bytes);
        let mut buf = [0; 64];
        let mut out_buf = RefWriteBuffer::new(&mut buf);

        aes.encrypt(&mut data_buf, &mut out_buf, true);

        let mut data_buf = out_buf.take_read_buffer();

        let mut data_bytes = data_buf.take_remaining();
        let data_string: String =
            bytes.iter().map(ToString::to_string).collect();
    
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
        //let data_string = data.to_string();
        
        let bytes = [data.to_le_bytes()[0]];
        println!("INTERGERS, INPUT DATA AS STRING {}\n INPUT DATA AS BYTES {:?}, BYTE STRING {:?}", data, bytes, data.to_be_bytes());
        let mut data_buf = RefReadBuffer::new(&bytes);
        let mut buf = [0; 16];
        let mut out_buf = RefWriteBuffer::new(&mut buf);

        let ak = "my-tao-testing-key".to_string();
        let mut aes = ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00'; 16]);
       
        aes.decrypt(&mut data_buf, &mut out_buf, false);
    
        let mut data_buf = out_buf.take_read_buffer();
        let mut data_bytes = data_buf.take_remaining();
        println!("OUTPUT DATA BUF {:?}", data_bytes);
    
        return data_bytes[0].into();
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
        let ak = "my-tao-testing-key".to_string();
        
        let mut aes =
            ctr(KeySize::KeySize256, &ak.into_bytes(), &[b'\x00'; 16]);
        
        let mut bytes = data.as_bytes();
        println!("STRING, INPUT DATA AS STRING {}\n INPUT DATA AS BYTES {:?}", data, bytes);
        let mut data_buf = RefReadBuffer::new(&bytes);
        let mut buf = [0; 16];
        let mut out_buf = RefWriteBuffer::new(&mut buf);

        aes.decrypt(&mut data_buf, &mut out_buf, true);

        let mut data_buf = out_buf.take_read_buffer();

        let mut data_bytes = data_buf.take_remaining();

        
        let data_string: String = data_bytes.iter().map(ToString::to_string).collect();
    
        println!("STRING, OUTPUT as bytes {:?}\n OUTPUT AS STRING {}", data_bytes, data_string);
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
