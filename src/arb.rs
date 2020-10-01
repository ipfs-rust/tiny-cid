use std::convert::TryFrom;

use quickcheck::{Arbitrary, Gen};
use rand::seq::SliceRandom;
use rand::Rng;
use tiny_multihash::{Code, Multihash, MultihashCode, U64};

use crate::codec::*;
use crate::{Cid, Version};

const CODECS: [u64; 18] = [
    RAW,
    DAG_PROTOBUF,
    DAG_CBOR,
    DAG_JSON,
    GIT_RAW,
    ETHEREUM_BLOCK,
    ETHEREUM_BLOCK_LIST,
    ETHEREUM_TX_TRIE,
    ETHEREUM_TX,
    ETHEREUM_TX_RECEIPT_TRIE,
    ETHEREUM_RECEIPT,
    ETHEREUM_STATE_TRIE,
    ETHEREUM_ACCOUNT_SNAPSHOT,
    ETHEREUM_STORAGE_TRIE,
    BITCOIN_BLOCK,
    BITCOIN_TX,
    ZCASH_BLOCK,
    ZCASH_TX,
];

const POPULAR: [u64; 4] = [RAW, DAG_PROTOBUF, DAG_CBOR, DAG_JSON];

impl Arbitrary for Version {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let version = if g.gen_bool(0.7) { 1 } else { 0 };
        Version::try_from(version).unwrap()
    }
}

impl Arbitrary for Cid {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let version: Version = Arbitrary::arbitrary(g);
        if version == Version::V0 {
            let data: Vec<u8> = Arbitrary::arbitrary(g);
            let hash = Code::Sha2_256.digest(&data);
            Cid::new_v0(hash).expect("sha2_256 is a valid hash for cid v0")
        } else {
            // chose the most frequently used codecs more often
            let codec = if g.gen_bool(0.7) {
                *POPULAR.choose(g).unwrap()
            } else {
                *CODECS.choose(g).unwrap()
            };
            let hash: Multihash<U64> = Arbitrary::arbitrary(g);
            Cid::new_v1(codec, hash)
        }
    }
}
