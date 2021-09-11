use std::str::FromStr;

use bitcoin::util::bip32::ExtendedPubKey;
use indicatif::{ProgressBar, ProgressStyle};

mod cli;
mod vanity;

fn main() -> Result<(), String> {
    let matches = cli::matches();
    let xpub_str = matches
        .value_of(cli::XPUB)
        .expect("xpub is a required field");
    let xpub_str =
        xyzpub::convert_version(xpub_str, &xyzpub::Version::Xpub).expect("xpub is invalid");

    let xpub = ExtendedPubKey::from_str(&xpub_str).expect("xpub is invalid");
    let prefixes: Vec<&str> = matches
        .values_of(cli::PREFIX)
        .expect("Prefix is a required field")
        .collect();

    let progress = create_spinner();
    let (path, address) = vanity::search_address(&xpub, &prefixes)?;
    progress.finish_and_clear();

    println!("Found address {} at path {}", &address, &path);

    Ok(())
}

/// Creates a spinner to indicate that the program is still running. Shows "Searching address..." with moving dots
fn create_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(240);
    spinner.set_message("Searching address");
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[".  ", ".. ", "...", "   "])
            .template("{elapsed_precise} {msg}{spinner}"),
    );

    spinner
}
