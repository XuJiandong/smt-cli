mod blake2b;

use blake2b::Blake2bHasher;
use clap::{App, AppSettings, Arg};
use lazy_static::lazy_static;

use smt::default_store::DefaultStore;
use smt::{SparseMerkleTree, H256};

lazy_static! {
    pub static ref NON_INCLUSION: H256 = [0; 32].into();
    pub static ref INCLUSION: H256 = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0
    ]
    .into();
}
const DELIMITER: &str = "|";

fn hex_format(hash: &[u8]) -> String {
    let strs: Vec<String> = hash.into_iter().map(|h| format!("{:02x}", h)).collect();
    String::from("0x") + &strs.join("")
}

fn array_format(hash: &[u8]) -> String {
    let strs: Vec<String> = hash.into_iter().map(|h| format!("{}", h)).collect();
    strs.join(",")
}

// sample
// 1|2|3
fn parse_index(arg: &str) -> Vec<usize> {
    let arg = arg.trim();
    fn parse(s: &str) -> usize {
        let res = s.trim().parse::<usize>();
        if res.is_err() {
            println!(
                "error: \"{}\" is not a number: please delimited number by |.",
                s
            );
            panic!("stop");
        } else {
            res.unwrap()
        }
    }
    arg.split(DELIMITER).into_iter().map(|i| parse(i)).collect()
}

// append 0 at the end
fn parse_hash(arg: &str) -> H256 {
    let arg = arg.trim().to_lowercase();
    let mut result = [0; 32];
    fn parse(c: &[u8]) -> u8 {
        let str = String::from_utf8_lossy(c);
        let res = u8::from_str_radix(str.as_ref(), 16);
        if res.is_err() {
            println!("error, \"{}\" is not a valid hex", str);
            panic!("stop");
        } else {
            res.unwrap()
        }
    }

    if arg.len() > 2 && "0x" == &arg[0..2] {
        let arg2 = arg.split_at(2).1;
        let r: Vec<u8> = arg2.as_bytes().chunks(2).map(|c| parse(c)).collect();
        &result[..r.len()].copy_from_slice(r.as_slice());
    } else {
        let r: Vec<u8> = arg
            .replace(" ", "")
            .split(",")
            .into_iter()
            .map(|i| i.parse::<u8>().unwrap())
            .collect();
        &result[..r.len()].copy_from_slice(r.as_slice());
    }
    result.into()
}

// sample
// 0xFF|0xEEFF00|...
// 12,23,45|0|255,255,255,255|...
// 0xFF|12|0
fn parse_hashes(arg: &str) -> Vec<H256> {
    let arg = arg.trim();
    arg.split(DELIMITER)
        .into_iter()
        .map(|i| parse_hash(i))
        .collect()
}

#[test]
fn test_parse_hashes() {
    let arg = "0xFF|0x0|0x1234567890";
    let hashes = parse_hashes(arg);
    assert_eq!(
        hashes,
        vec![
            [
                0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
            .into(),
            [0; 32].into(),
            [
                0x12, 0x34, 0x56, 0x78, 0x90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
            .into()
        ]
    );
    let arg2 = "0xFF|0|12,34,56,78,90";
    let hashes2 = parse_hashes(arg2);
    assert_eq!(
        hashes2,
        vec![
            [
                0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
            .into(),
            [0; 32].into(),
            [
                12, 34, 56, 78, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0
            ]
            .into()
        ]
    );
}

#[test]
fn test_parse_hashes2() {
    let arg = "255 | 0 |12,34,56,78,90";
    let hashes = parse_hashes(arg);
    assert_eq!(
        hashes,
        vec![
            [
                255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
            .into(),
            [0; 32].into(),
            [
                12, 34, 56, 78, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0
            ]
            .into()
        ]
    );
}

#[test]
fn test_parse_index() {
    let arg = "0|1|2";
    let indexes = parse_index(arg);
    assert_eq!(indexes, vec![0, 1, 2]);
}

fn main() {
    let matches = App::new("smt-cli")
        .version("1.0")
        .about("Sparse Merkel Tree command line tool")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("include")
                .required(false)
                .short("i")
                .long("include")
                .value_name("INDEXES")
                .takes_value(true)
                .help("Indexes of hashes to include, e.g. 1 | 2 | 3, delimited by |"),
        )
        .arg(
            Arg::with_name("exclude")
                .required(false)
                .short("e")
                .long("exclude")
                .value_name("HASHES")
                .takes_value(true)
                .help("Hashes to exclude, delimited by |"),
        )
        .arg(
            Arg::with_name("kvpair")
                .required(false)
                .short("k")
                .long("kvpair")
                .help("By default, assume the value is [1, 0, 0, ...]. When set, value provided."),
        )        // trailing args
        .arg(
            Arg::with_name("hex")
                .required(false)
                .short("x")
                .long("hex")
                .help("print data in hex string format, starting with 0x. The default is array string format."),
        )        // trailing args
        .arg(Arg::with_name("hashes")
            .value_name("HASHES")
            .multiple(true)
        )
        .get_matches();

    let include = matches.value_of("include");
    let exclude = matches.value_of("exclude");
    let is_hex_format = matches.is_present("hex");
    let is_kvpair = matches.is_present("kvpair");

    if include.is_some() && exclude.is_some() {
        println!("include and exclude can't be both used.");
        println!("{}", matches.usage());
        return;
    } else if include.is_none() && exclude.is_none() {
        println!("must specify include or exclude.");
        println!("{}", matches.usage());
        return;
    }
    let hashes_str: Vec<&str> = matches.values_of("hashes").unwrap().collect();
    let hashes: Vec<H256> = hashes_str.into_iter().map(|h| parse_hash(h)).collect();

    if is_kvpair {
        if hashes.len() % 2 != 0 {
            println!("The arguments count should be even: <hash as key 1> <value 1> <hash as key 2> <value 2> ...");
            panic!("stop");
        }
    }

    let key_hashes: Vec<H256> = if is_kvpair {
        hashes
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, v)| v)
            .collect()
    } else {
        hashes.clone()
    };
    let value_hashes: Vec<H256> = if is_kvpair {
        hashes
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, v)| v)
            .collect()
    } else {
        let mut res: Vec<H256> = vec![];
        for _ in 0..hashes.len() {
            res.push(INCLUSION.clone());
        }
        res
    };

    let (root, proof) = if include.is_some() {
        let index = parse_index(include.unwrap());
        make_inclusion_proof(key_hashes, value_hashes, index)
    } else {
        assert!(exclude.is_some());
        let non_inclusion_hashes = parse_hashes(exclude.unwrap());
        make_none_inclusion_proof(key_hashes, value_hashes, non_inclusion_hashes)
    };
    if is_hex_format {
        println!("root: {}", hex_format(root.as_slice()));
        println!("proof: {}", hex_format(proof.as_slice()));
    } else {
        println!("root: {}", array_format(root.as_slice()));
        println!("proof: {}", array_format(proof.as_slice()));
    }
}

type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

fn new_smt(pairs: Vec<(H256, H256)>) -> SMT {
    let mut smt = SMT::default();
    for (key, value) in pairs {
        smt.update(key, value).unwrap();
    }
    smt
}

fn make_inclusion_proof(
    key_hashes: Vec<H256>,
    value_hashes: Vec<H256>,
    indexes: Vec<usize>,
) -> (H256, Vec<u8>) {
    assert_eq!(key_hashes.len(), value_hashes.len());

    let kv_hashes: Vec<(H256, H256)> = key_hashes
        .clone()
        .into_iter()
        .zip(value_hashes.clone())
        .collect();
    let smt = new_smt(kv_hashes.clone());
    let root = smt.root();

    let inclusion_pairs: Vec<(H256, H256)> = indexes
        .into_iter()
        .map(|i| (key_hashes[i], value_hashes[i]))
        .collect();
    let inclusion_keys = inclusion_pairs
        .clone()
        .into_iter()
        .map(|(k, _)| k)
        .collect();

    let proof = smt.merkle_proof(inclusion_keys).expect("gen proof");
    let compiled_proof = proof.clone().compile(inclusion_pairs.clone()).unwrap();
    // verify it locally
    assert!(compiled_proof
        .verify::<Blake2bHasher>(smt.root(), inclusion_pairs)
        .expect("verify compiled proof"));

    (root.clone(), compiled_proof.0)
}

fn make_none_inclusion_proof(
    key_hashes: Vec<H256>,
    value_hashes: Vec<H256>,
    non_inclusion_keys: Vec<H256>,
) -> (H256, Vec<u8>) {
    let kv_hashes: Vec<(H256, H256)> = key_hashes
        .clone()
        .into_iter()
        .zip(value_hashes.clone())
        .collect();
    let smt = new_smt(kv_hashes.clone());
    let root = smt.root();

    let proof = smt
        .merkle_proof(non_inclusion_keys.clone())
        .expect("gen proof");
    let non_inclusion_pairs: Vec<(H256, H256)> = non_inclusion_keys
        .into_iter()
        .map(|i| (i, NON_INCLUSION.clone()))
        .collect();
    let compiled_proof = proof.clone().compile(non_inclusion_pairs.clone()).unwrap();
    // verify it locally
    assert!(compiled_proof
        .verify::<Blake2bHasher>(smt.root(), non_inclusion_pairs)
        .expect("verify compiled proof"));

    (root.clone(), compiled_proof.0)
}
