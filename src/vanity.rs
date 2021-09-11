use core::panic;

use bitcoin::{
    secp256k1::{All, Secp256k1},
    util::bip32::{ChildNumber, DerivationPath, ExtendedPubKey},
    Address, Network,
};
use rayon::prelude::*;

const MAX_INDEX: u32 = 2 << 31 - 1;

pub fn search_address(xpub: &ExtendedPubKey, prefixes: &[&str]) -> (DerivationPath, Address) {
    let mut root = IncrementablePath::new();
    let mut found_path = None::<DerivationPath>;
    let secp = Secp256k1::new();

    while found_path.is_none() {
        found_path = root
            .path()
            .normal_children()
            .par_bridge()
            .find_any(|path| check_path(&secp, xpub, prefixes, path));

        if found_path.is_none() {
            root.increment();
            println!("Incremented to: {:?}", &root.0);
        }
    }

    // TODO: Separate method to avoid duplicate code check_path
    if let Some(path) = found_path {
        if let Ok(derived) = xpub.derive_pub(&secp, &path) {
            if let Ok(address) = Address::p2wpkh(&derived.public_key, Network::Bitcoin) {
                return (path, address);
            }
        }
    }
    panic!();
}

struct IncrementablePath(Vec<u32>);

impl IncrementablePath {
    fn new() -> Self {
        Self(vec![0u32])
    }

    fn increment(&mut self) {
        for index in self.0.iter_mut().rev() {
            if index == &MAX_INDEX {
                *index = 0;
            } else {
                *index += 1;
                return;
            }
        }

        // Increase depth because every index was MAX
        self.0.push(0);
    }

    fn path(&self) -> DerivationPath {
        let path: Vec<ChildNumber> = self
            .0
            .iter()
            .map(|i| ChildNumber::from_normal_idx(*i).expect("Index is not in valid index range"))
            .collect();
        DerivationPath::from(path)
    }
}

fn check_path(
    secp: &Secp256k1<All>,
    xpub: &ExtendedPubKey,
    prefixes: &[&str],
    path: &DerivationPath,
) -> bool {
    if let Ok(derived) = xpub.derive_pub(secp, path) {
        if let Ok(address) = Address::p2wpkh(&derived.public_key, Network::Bitcoin) {
            let addr_str = address.to_string();
            prefixes.iter().any(|prefix| addr_str.starts_with(prefix))
        } else {
            panic!(
                "Failed create address from public key {}\n",
                &derived.public_key
            );
        }
    } else {
        panic!("Failed to derive path {}\n", &path);
    }
}

// TODO: Remove panics and println!
// TODO: Docs
// TODO: Tests
// TODO: Skip bc1q prefix
