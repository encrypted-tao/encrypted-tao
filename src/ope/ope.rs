/*
 * File: ope.rs
 *      Rust implementation of Order Preserving Encryption OPE
 *      Based off of a python implementation
 *      (https://github.com/tonyo/pyope/blob/master/pyope/ope.py)
 *
 *      Ref:
 *          https://faculty.cc.gatech.edu/~aboldyre/papers/bclo.pdf
 *          https://people.csail.mit.edu/nickolai/papers/popa-mope-eprint.pdf
 *          https://arxiv.org/pdf/2009.05679.pdf
 *          https://github.com/husobee/ope
 *
 */

extern crate crypto; // https://github.com/RustCrypto/traits/tree/master/crypto
extern crate hmac;

pub mod ope {

    use crate::ope::hgd::{hypergeo_sample, PRNG};
    use crate::ope::stats::uniform_sample;
    use crate::ope::utils::aes_init;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    pub struct Range {
        pub start: u64,
        pub end: u64,
    }

    impl Range {
        pub fn contains(&mut self, number: u64) -> bool {
            return self.start <= number && self.end >= number;
        }

        pub fn size(&mut self) -> u64 {
            if self.end.checked_sub(self.start).is_some() {
                return self.end - self.start + 1;
            }
            return 1;
        }

        pub fn copy(&mut self) -> Range {
            return Range {
                start: self.start,
                end: self.end,
            };
        }
    }

    pub struct OPE {
        pub key: String,
        pub in_range: Range,
        pub out_range: Range,
    }

    impl OPE {
        /*
         * encrypt(self, plaintext)
         *  OPE is recursive encryption, check cases then recursive helper
         */
        pub fn encrypt(&mut self, plaintext: u64) -> u64 {
            if !self.in_range.contains(plaintext) {
                println!("range does not contain plaintext\n");
                return 1 as u64;
            }

            return self.recursive_encrypt(
                plaintext,
                self.in_range.start,
                self.in_range.end,
                self.out_range.start,
                self.out_range.end,
            );
        }

        pub fn recursive_encrypt(
            &mut self,
            plaintext: u64,
            in_start: u64,
            in_end: u64,
            out_start: u64,
            out_end: u64,
        ) -> u64 {
            let mut in_range = Range {
                start: in_start,
                end: in_end,
            };
            let mut out_range = Range {
                start: out_start,
                end: out_end,
            };
            let in_size = in_range.size();
            let out_size = out_range.size();
            let mut in_edge = in_range.start;
            if in_range.start.checked_sub(1).is_some() {
                in_edge -= 1;
            }

            let mut out_edge = out_range.start;
            if out_range.start.checked_sub(1).is_some() {
                out_edge -= 1;
            }
            let mut mid = out_edge + (out_size / 2) as u64;

            // sanity check
            assert!(in_size <= out_size);

            if in_range.size() == 1 {
                let output = self.tape_gen(plaintext);
                let ciphertext = uniform_sample(out_range, output);
                return ciphertext;
            }

            let output = self.tape_gen(mid);

            let mut samples = hypergeo_sample(
                in_start, in_end, out_start, out_end, mid, output,
            );

            if plaintext <= samples {
                if in_edge.checked_add(1).is_some() {
                    in_edge += 1;
                }
                if out_edge.checked_add(1).is_some() {
                    out_edge += 1;
                }
                return self.recursive_encrypt(
                    plaintext, in_edge, samples, out_edge, mid,
                );
            } else {
                if samples.checked_add(1).is_some() {
                    samples += 1;
                }
                if in_edge.checked_add(in_size).is_some() {
                    in_edge += in_size;
                }
                if mid.checked_add(1).is_some() {
                    mid += 1;
                }
                if out_edge.checked_add(out_size).is_some() {
                    out_edge += out_size;
                }
                return self.recursive_encrypt(
                    plaintext, samples, in_edge, mid, out_edge,
                );
            }
        }
        pub fn decrypt(&mut self, ciphertext: u64) -> u64 {
            if !self.out_range.contains(ciphertext) {
                println!("range does not contain ciphertext\n");
                return 1 as u64;
            }
            return self.recursive_decrypt(
                ciphertext,
                self.in_range.start,
                self.in_range.end,
                self.out_range.start,
                self.out_range.end,
            );
        }

        pub fn recursive_decrypt(
            &mut self,
            ciphertext: u64,
            in_start: u64,
            in_end: u64,
            out_start: u64,
            out_end: u64,
        ) -> u64 {
            let mut in_range = Range {
                start: in_start,
                end: in_end,
            };
            let mut out_range = Range {
                start: out_start,
                end: out_end,
            };
            let in_size = in_range.size();
            let out_size = out_range.size();

            let mut in_edge = in_range.start;
            if in_range.start.checked_sub(1).is_some() {
                in_edge -= 1;
            }

            let mut out_edge = out_range.start;
            if out_range.start.checked_sub(1).is_some() {
                out_edge -= 1;
            }
            let mut mid = out_edge + (out_size / 2) as u64;

            // sanity check
            assert!(in_size <= out_size);

            if in_range.size() == 1 {
                return in_range.start;
            }

            let output = self.tape_gen(mid);

            let mut samples = hypergeo_sample(
                in_start, in_end, out_start, out_end, mid, output,
            );

            if ciphertext <= mid {
                if in_edge.checked_add(1).is_some() {
                    in_edge += 1;
                }
                if out_edge.checked_add(1).is_some() {
                    out_edge += 1;
                }
                return self.recursive_decrypt(
                    ciphertext, in_edge, samples, out_edge, mid,
                );
            } else {
                if samples.checked_add(1).is_some() {
                    samples += 1;
                }
                if in_edge.checked_add(in_size).is_some() {
                    in_edge += in_size;
                }
                if mid.checked_add(1).is_some() {
                    mid += 1;
                }
                if out_edge.checked_add(out_size).is_some() {
                    out_edge += out_size;
                }
                return self.recursive_decrypt(
                    ciphertext, samples, in_edge, mid, out_edge,
                );
            }
        }

        /*
         * tape_gen(self, data)
         *  Return: PRNG (cipher specifically)
         */
        pub fn tape_gen(&mut self, data: u64) -> PRNG {
            let data_str = data.to_string();
            let data_bytes = data_str.as_bytes();

            type HmacSha256 = Hmac<Sha256>;

            let mut hmac_obj =
                HmacSha256::new_from_slice(self.key.as_bytes()).unwrap();

            hmac_obj.update(&data_bytes);

            let hmac_res = hmac_obj.finalize();

            let cipher = aes_init(&mut hmac_res.clone().into_bytes());

            let prng = PRNG {
                cipher: cipher,
                tape: [0; 96],
            };

            return prng;
        }
    }
}
/*
 * OPE tests
 *  run via `cargo test`
 */
#[cfg(test)]
mod tests {

    use crate::ope::ope::ope::Range;
    use crate::ope::ope::ope::OPE;

    pub const DEFAULT_INPUT_RANGE_END: u64 = u16::max_value() as u64 - 1;
    pub const DEFAULT_OUTPUT_RANGE_END: u64 = u32::max_value() as u64 - 1;

    #[test]
    fn test_encrypt() {
        let mut test = OPE {
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
        let tests: [u64; 3] = [10, 100, 1000];
        let res: [u64; 3] = [131086, 4747723, 60293123];

        for i in 0..3 {
            let encrypt = test.encrypt(tests[i]);
            assert_eq!(res[i], encrypt);
        }
    }

    #[test]
    fn test_ordering() {
        let mut test = OPE {
            key: "testing-key".to_string(),
            in_range: Range {
                start: 0 as u64,
                end: DEFAULT_INPUT_RANGE_END,
            },
            out_range: Range {
                start: 0 as u64,
                end: DEFAULT_OUTPUT_RANGE_END,
            },
        };
        let a = test.encrypt(13 as u64);
        let b = test.encrypt(14 as u64);
        let c = test.encrypt(15 as u64);
        println!("result of a: {}, b: {}, c: {}", a, b, c);

        assert!(a < b);
        assert!(b < c);
    }

    #[test]
    fn test_decrypt() {
        let mut test = OPE {
            key: "ope-testing-key".to_string(),
            in_range: Range {
                start: 0,
                end: DEFAULT_INPUT_RANGE_END,
            },
            out_range: Range {
                start: 0,
                end: DEFAULT_OUTPUT_RANGE_END,
            },
        };

        let tests: [u64; 4] = [25, 50, 75, 750];
        for i in 0..4 {
            let encrypt = test.encrypt(tests[i]);
            assert_eq!(tests[i], test.decrypt(encrypt));
        }
    }
}
