# Simplee > Aabel >> Multi-Hash

[![Crates.io][crates-badge]][crates-url]
[![CI][ci-badge]][ci-url]
![GitHub top language][lang-badge]
[![License:MIT][license-badge]][license-url]
![GitHub code size in bytes][size-badge]
![GitHub last commit][last-commit-badge]
![GitHub watchers][watchers-badge]

A crate which extends [Hasher][hasher_url] and [BuildHasher][buildhasher_url] traits. These extensions can be used in algorithms, eg streaming ones, which require several hashing functions to be executed for each incoming item. As example, bloom filter and count-min use such hash values to represent the incoming items.

## HasherExt Trait
The **HasherExt** trait extends the [Hasher][hasher_url] trait by adding capabilities to finalize the hashing operation and getting back an infinite sequest of hash values.
The crate provides **PairHasher** which implements the **HasherExt** trait. The **PairHasher** is a combinator of two separate hashers, which are used to obtain
the sequece of hash values, each value representing the result of an independent hashing function.

## BuildHasherExt Trait
The **BuildHasherExt** trait extends the [BuildHasher][buildhasher_url] trait. It adds a cabapility to internally build an instance of the **HasherExt** trait which is used to generate the sequence of the hash values. The **BuildHasherExt** trait exposes the *hashes_one* function, which is the one that takes as input an item and returns the sequence of the hash values.

## Example

```rust
use aabel_multihash_rs::{BuildHasherExt, PairHasherBuilder};
use std::hash::{BuildHasher, Hash};

// Create the hasher builder
let keys1 = (0, 0);
let keys2 = (1, 1);
let builder = PairHasherBuilder::new_with_keys(keys1, keys2);

// The number of hash functions
const HASH_COUNT: usize = 10;

// Compute 10 hash values
let item = "Hello world!";
let hashes = builder
    .hashes_one(item)
    .take(HASHE_COUNT)
    .collect::<Vec<_>>();

assert_eq!(hashes.len(), HASHE_COUNT)
```

## About
> Code designed and written on the beautiful island of [**Saaremaa**][estonia], Estonia.

[crates-badge]: https://img.shields.io/crates/v/aabel-multihash-rs.svg
[crates-url]: https://crates.io/crates/aabel-multihash-rs
[ci-badge]: https://github.com/veminovici/aabel-multihash-rs/actions/workflows/ci.yml/badge.svg?branch=main
[ci-url]: https://github.com/veminovici/aabel-multihash-rs/actions/workflows/ci.yml
[lang-badge]: https://img.shields.io/github/languages/top/veminovici/aabel-multihash-rs
[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: https://opensource.org/licenses/MIT
[size-badge]: https://img.shields.io/github/languages/code-size/veminovici/aabel-multihash-rs
[last-commit-badge]: https://img.shields.io/github/last-commit/veminovici/aabel-multihash-rs
[watchers-badge]: https://img.shields.io/github/watchers/veminovici/aabel-multihash-rs
[estonia]: https://goo.gl/maps/DmB9ewY2R3sPGFnTA
[hasher_url]: https://doc.rust-lang.org/std/hash/trait.Hasher.html
[buildhasher_url]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html