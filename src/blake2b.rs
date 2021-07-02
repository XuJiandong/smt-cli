#![allow(non_camel_case_types)]
use blake2b_rs::{Blake2b, Blake2bBuilder};

use rsmt::{traits::Hasher, H256};
use smt::{traits::Hasher as Hasher_2, H256 as H256_2};

const BLAKE2B_KEY: &[u8] = &[];
const BLAKE2B_LEN: usize = 32;
const PERSONALIZATION: &[u8] = b"ckb-default-hash";

pub struct Blake2bHasher(Blake2b);

impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2bBuilder::new(BLAKE2B_LEN)
            .personal(PERSONALIZATION)
            .key(BLAKE2B_KEY)
            .build();
        Blake2bHasher(blake2b)
    }
}

impl Hasher for Blake2bHasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }
    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        self.0.finalize(&mut hash);
        hash.into()
    }
}

pub struct Blake2bHasher_2(Blake2b);

impl Default for Blake2bHasher_2 {
    fn default() -> Self {
        let blake2b = Blake2bBuilder::new(BLAKE2B_LEN)
            .personal(PERSONALIZATION)
            .key(BLAKE2B_KEY)
            .build();
        Blake2bHasher_2(blake2b)
    }
}

impl Hasher_2 for Blake2bHasher_2 {
    fn write_h256(&mut self, h: &H256_2) {
        self.0.update(h.as_slice());
    }
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }
    fn finish(self) -> H256_2 {
        let mut hash = [0u8; 32];
        self.0.finalize(&mut hash);
        hash.into()
    }
}
