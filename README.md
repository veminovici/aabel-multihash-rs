# Simplee > Aabel >> Multi-Hash

![GitHub top language][lang-badge]
[![License:MIT][license-badge]][license-url]
![GitHub code size in bytes][size-badge]
![GitHub last commit][last-commit-badge]
![GitHub watchers][watchers-badge]

A crate which extends **Hasher** and **BuildHasher** traits. These extensions can be used in algorithms, eg streaming ones, which require several hashing functions to be executed for each incoming item. As example, bloom filter and count-min use such hash values to represent the incoming items.

## HasherExt Trait
The **HasherExt** trait extends the **Hasher** trait by adding capabilities to finalize the hashing operation and getting back an infinite sequest of hash values.
The crate provides **PairHasher** which implements the **HasherExt** trait. The **PairHasher** is a combinator of two separate hashers, which are used to obtain
the sequece of hash values, each value representing the result of an independent hashing function.

## BuildHasherExt Trait
The **BuildHasherExt** trait extends the **BuildHasher** trait. It adds a cabapility to internally build an instance of the **HasherExt** trait which is used to generate the sequence of the hash values. The **BuildHasherExt** trait exposes the *hashes_one* function, which is the one that takes as input an item and returns the sequence of the hash values.

## Example

```rust
use aabel_multihash_rs::{BuildHasherExt, PairHasherBuilder};
use std::hash::{BuildHasher, Hash};

// Create the hasher builder
let keys1 = (0, 0);
let keys2 = (1, 1);
let builder = PairHasherBuilder::new_with_keys(keys1, keys2);

// The number of hash functions.
const HASH_COUNT: usize = 10;

let item = "Hello world!";
let hashes = builder
    .hashes_one(item)
    .take(HASHE_COUNT)
    .collect::<Vec<_>>();

assert_eq!(hashes.len(), HASHE_COUNT)
```

## About
> Code designed and written on the beautiful island of [**Saaremaa**][estonia], Estonia.

[estonia]: https://goo.gl/maps/DmB9ewY2R3sPGFnTA