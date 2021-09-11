use std::str::FromStr;

use bitcoin::util::bip32::ExtendedPubKey;

mod cli;
mod vanity;

fn main() {
    let matches = cli::matches();
    let xpub_str = matches
        .value_of(cli::XPUB)
        .expect("xpub is a required field");
    let xpub = ExtendedPubKey::from_str(xpub_str).expect("xpub is not valid");
    let prefixes: Vec<&str> = matches
        .values_of(cli::PREFIX)
        .expect("Prefix is a required field")
        .collect();

    let (path, address) = vanity::search_address(&xpub, &prefixes);

    println!("Found address {} at path {}", &address, &path);
}

// TODO: indicatif with path counter
