//! A crate which defines two extensions:
//! - [`HasherExt`] trait which extends the [`Hasher`] trait.
//! - [`BuildHasherExt`] trait which extends the [`BuildHasher`] trait.
//!
//! The purpose of these extensions is to allow the callers to compute
//! sequences of hash values for any item that is hashable. This functionality
//! is used in different algorithms such probabilistic data structures.
//!
//! The [`PairHasher`] implements the [`HasherExt`] trait. It a combinator of two [`Hasher`] instances which are used in order to generate the sequence of hash values.
//!
//! The [`PairHasherBuilder`] implements the [`BuildHasherExt`] trait. It provides a convenient way to build [`PairHasher`] instances. It also prives convenient *new* functions
//! which allow the user to create [`PairHasher`] instances by combining two [`siphasher::sip::SipHasher`] instances.
//!
//! # Example
//!
//!```
//! use aabel_multihash_rs::{BuildHasherExt, PairHasherBuilder};
//! use std::hash::{BuildHasher, Hash};
//!
//! let keys1 = (0, 0);
//! let keys2 = (1, 1);
//! let builder = PairHasherBuilder::new_with_keys(keys1, keys2);
//!
//! const HASHE_COUNT: usize = 10;
//!
//! let item = "Hello world!";
//! let hashes = builder
//!    .hashes_one(item)
//!    .take(HASHE_COUNT)
//!    .collect::<Vec<_>>();
//!
//! assert_eq!(hashes.len(), HASHE_COUNT)
//!```
use std::hash::{BuildHasher, Hash, Hasher};

mod build_siphasher;
mod hash_iter;
mod pairhasher;

pub use pairhasher::*;

/// Extends the [`Hasher`] trait by providing a mechanism to
/// get a sequence of hash values when the hashing operation is finalized.
pub trait HasherExt: Hasher {
    /// Returns an **infinite** sequence of hash values for the values written so far.
    /// NB: Before you call collect on this iterator, please make sure you have reduce the
    /// collection to a finite sequece by calling take, for example.
    ///
    /// Its behavior it is different than the [`Hasher::finish`]s one. The method consumes
    /// the hasher instance, so to generate new hashes you need to rebuild the hasher instance.
    fn finish_iter(self) -> impl Iterator<Item = u64>;
}

/// Extends the [`BuildHasher`] trait by allowing to compute the sequence of hash values
/// for one given hashable value.
pub trait BuildHasherExt: BuildHasher {
    /// Generates the sequece of hash values for a given item.
    fn hashes_one<T: Hash>(&self, item: T) -> impl Iterator<Item = u64>
    where
        Self: Sized,
        Self::Hasher: HasherExt,
    {
        let mut hasher = self.build_hasher();

        item.hash(&mut hasher);
        hasher.finish_iter()
    }
}

impl<T> BuildHasherExt for T
where
    T: BuildHasher,
    <T as BuildHasher>::Hasher: HasherExt,
{
}
