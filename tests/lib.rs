use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use multibase::Base;
use tiny_cid::{Cid, Version, DAG_PROTOBUF, RAW};
use tiny_multihash::{Code, MultihashCode};

#[test]
fn basic_marshalling() {
    let h = Code::Sha2_256.digest(b"beep boop");

    let cid = Cid::new_v1(DAG_PROTOBUF, h);

    let data = cid.to_bytes();
    let out = Cid::try_from(data.clone()).unwrap();
    assert_eq!(cid, out);

    let out2: Cid = data.try_into().unwrap();
    assert_eq!(cid, out2);

    let s = cid.to_string();
    let out3 = Cid::try_from(&s[..]).unwrap();
    assert_eq!(cid, out3);

    let out4: Cid = (&s[..]).try_into().unwrap();
    assert_eq!(cid, out4);
}

#[test]
fn empty_string() {
    assert!(Cid::try_from("").is_err());
}

#[test]
fn v0_handling() {
    let old = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";
    let cid = Cid::try_from(old).unwrap();

    assert_eq!(cid.version(), Version::V0);
    assert_eq!(cid.to_string(), old);
}

#[test]
fn from_str() {
    let cid: Cid = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n"
        .parse()
        .unwrap();
    assert_eq!(cid.version(), Version::V0);

    let bad = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zIII".parse::<Cid>();
    assert!(bad.is_err());
}

#[test]
fn v0_error() {
    let bad = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zIII";
    assert!(Cid::try_from(bad).is_err());
}

#[test]
fn from() {
    let the_hash = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    let cases = vec![
        format!("/ipfs/{:}", &the_hash),
        format!("https://ipfs.io/ipfs/{:}", &the_hash),
        format!("http://localhost:8080/ipfs/{:}", &the_hash),
    ];

    for case in cases {
        let cid = Cid::try_from(case).unwrap();
        assert_eq!(cid.version(), Version::V0);
        assert_eq!(cid.to_string(), the_hash);
    }
}

#[test]
fn test_hash() {
    let data: [u8; 3] = [1, 2, 3];
    let hash = Code::Sha2_256.digest(&data);
    let mut map = HashMap::new();
    let cid = Cid::new_v0(hash).unwrap();
    map.insert(cid, data);
    assert_eq!(&data, map.get(&cid).unwrap());
}

#[test]
fn test_base32() {
    let cid = Cid::from_str("bafkreibme22gw2h7y2h7tg2fhqotaqjucnbc24deqo72b6mkl2egezxhvy").unwrap();
    let hash = Code::Sha2_256.digest(b"foo");
    assert_eq!(cid.version(), Version::V1);
    assert_eq!(cid.codec(), RAW);
    assert_eq!(cid.hash(), &hash);
}

#[test]
fn to_string() {
    let expected_cid = "bafkreibme22gw2h7y2h7tg2fhqotaqjucnbc24deqo72b6mkl2egezxhvy";
    let hash = Code::Sha2_256.digest(b"foo");
    let cid = Cid::new_v1(RAW, hash);
    assert_eq!(cid.to_string(), expected_cid);
}

#[test]
fn to_string_of_base32() {
    let expected_cid = "bafkreibme22gw2h7y2h7tg2fhqotaqjucnbc24deqo72b6mkl2egezxhvy";
    let hash = Code::Sha2_256.digest(b"foo");
    let cid = Cid::new_v1(RAW, hash);
    assert_eq!(
        cid.to_string_of_base(Base::Base32Lower).unwrap(),
        expected_cid
    );
}

#[test]
fn to_string_of_base64() {
    let expected_cid = "mAVUSICwmtGto/8aP+ZtFPB0wQTQTQi1wZIO/oPmKXohiZueu";
    let hash = Code::Sha2_256.digest(b"foo");
    let cid = Cid::new_v1(RAW, hash);
    assert_eq!(cid.to_string_of_base(Base::Base64).unwrap(), expected_cid);
}

#[test]
fn to_string_of_base58_v0() {
    let expected_cid = "QmRJzsvyCQyizr73Gmms8ZRtvNxmgqumxc2KUp71dfEmoj";
    let hash = Code::Sha2_256.digest(b"foo");
    let cid = Cid::new_v0(hash).unwrap();
    assert_eq!(
        cid.to_string_of_base(Base::Base58Btc).unwrap(),
        expected_cid
    );
}

#[test]
fn to_string_of_base_v0_error() {
    let hash = Code::Sha2_256.digest(b"foo");
    let cid = Cid::new_v0(hash).unwrap();
    assert!(cid.to_string_of_base(Base::Base16Upper).is_err());
}
