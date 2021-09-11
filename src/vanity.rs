use core::panic;

use bitcoin::{
    secp256k1::{All, Secp256k1},
    util::bip32::{ChildNumber, DerivationPath, ExtendedPubKey},
    Address, Network,
};
use rayon::prelude::*;

const MAX_INDEX: u32 = 2 << 31 - 1;

/// Searches every derivation path in a breadth first search for an address starting with a `prefix`
pub fn search_address(
    xpub: &ExtendedPubKey,
    prefixes: &[&str],
) -> Result<(DerivationPath, Address), String> {
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
        }
    }

    if let Some(path) = found_path {
        if let Ok(derived) = xpub.derive_pub(&secp, &path) {
            if let Ok(address) = Address::p2wpkh(&derived.public_key, Network::Bitcoin) {
                return Ok((path, address));
            }
        }
    }

    Err("Failed to derive xpub at found path".to_string())
}

/// Wrapper for a derivation path vec that can only be incremented
struct IncrementablePath(Vec<u32>);

impl IncrementablePath {
    /// Constructor for path m/
    fn new() -> Self {
        Self(vec![])
    }

    /// Increments its derivation path respecting the maximum index value of 2^31 - 1
    fn increment(&mut self) {
        for index in self.0.iter_mut().rev() {
            if *index >= MAX_INDEX {
                *index = 0;
            } else {
                *index += 1;
                return;
            }
        }

        // Increase depth because every index was MAX
        self.0.push(0);
    }

    /// Returns its path as a [DerivationPath] with normal indexes
    fn path(&self) -> DerivationPath {
        let path: Vec<ChildNumber> = self
            .0
            .iter()
            .map(|i| ChildNumber::from_normal_idx(*i).expect("Index is not in valid index range"))
            .collect();
        DerivationPath::from(path)
    }
}

/// Checks if the address derived from a given path matches a prefix
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
                "Failed to create address from public key {}",
                &derived.public_key
            );
        }
    } else {
        panic!("Failed to derive path {}", &path);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::util::bip32::DerivationPath;

    use crate::vanity::MAX_INDEX;

    use super::IncrementablePath;

    #[test]
    fn incrementable_path_new() {
        let path = IncrementablePath::new();

        assert_eq!(path.path(), DerivationPath::master())
    }

    #[test]
    fn incrementable_path_increment() {
        let mut path_start = IncrementablePath(vec![0u32]);
        let mut path_end = IncrementablePath(vec![MAX_INDEX]);

        path_start.increment();
        path_end.increment();

        assert_eq!(path_start.path(), DerivationPath::from_str("m/1").unwrap());
        assert_eq!(path_end.path(), DerivationPath::from_str("m/0/0").unwrap());
    }
}
