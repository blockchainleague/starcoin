// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub use crypto_macro::{CryptoHash, CryptoHasher};
pub use libra_crypto::hash::{CryptoHash, CryptoHasher, DefaultHasher, HashValue, TestOnlyHash};

/// A type that implements `PlainCryptoHash` can be hashed by a cryptographic hash function and produce
/// a `HashValue`.
/// libra_crypto::hash::CryptoHash need a Hasher with a salt, this trait do not need hasher.
pub trait PlainCryptoHash {
    /// Hashes the object and produces a `HashValue`.
    fn crypto_hash(&self) -> HashValue;
}

///Auto implement `PlainCryptoHash` for libra_crypto::hash::CryptoHash
impl<C> PlainCryptoHash for C
where
    C: CryptoHash,
{
    fn crypto_hash(&self) -> HashValue {
        self.hash()
    }
}

pub fn create_literal_hash(word: &str) -> HashValue {
    let mut s = word.as_bytes().to_vec();
    assert!(s.len() <= HashValue::LENGTH);
    s.resize(HashValue::LENGTH, 0);
    HashValue::from_slice(&s).expect("Cannot fail")
}
