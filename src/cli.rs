use clap::{App, Arg, ArgMatches};

pub const XPUB: &str = "xpub";
pub const PREFIX: &str = "prefix";

pub fn matches() -> ArgMatches<'static> {
    App::new("senzu")
        .version("0.1.0")
        .about("Vanity address generator searching addresses derived from your xpub")
        .arg(
            Arg::with_name(XPUB)
                .help("xpub to derive addresses from")
                .long(XPUB)
                .required(true)
                .takes_value(true)
                .empty_values(false),
        )
        .arg(
            Arg::with_name(PREFIX)
                .help("Prefixes of your desired vanity address")
                .long(PREFIX)
                .required(true)
                .takes_value(true)
                .empty_values(false)
                .validator(check_prefix)
                .multiple(true),
        )
        .get_matches()
}

/// Checks if a given prefix is a valid bech32 bitcoin address prefix
fn check_prefix(prefix: String) -> Result<(), String> {
    if !prefix.starts_with("bc1q") {
        return Err("Prefix needs to start with \"bc1q\"".to_string());
    }
    if prefix.len() <= 4 {
        return Err("Prefix is too short".to_string());
    }
    if prefix.len() >= 25 {
        return Err("Prefix is too long".to_string());
    }
    if prefix[4..].chars().any(|c| "1bio".contains(c)) {
        return Err("Prefix can't have the characters '1', 'b', 'i' or 'o'".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cli::check_prefix;

    #[test]
    fn check_prefix_returns_err() {
        let too_short = "bc1q".to_string();
        let too_long = "bc1qqqqqqqqqqqqqqqqqqqqqqq".to_string();
        let invalid_char = vec![
            "bc1q1".to_string(),
            "bc1qb".to_string(),
            "bc1qi".to_string(),
            "bc1qo".to_string(),
        ];
        let wrong_prefix = "btc1q".to_string();

        let mut results = vec![
            check_prefix(too_short),
            check_prefix(too_long),
            check_prefix(wrong_prefix),
        ];
        invalid_char
            .into_iter()
            .for_each(|s| results.push(check_prefix(s)));

        for res in results {
            assert!(res.is_err())
        }
    }

    #[test]
    fn check_prefix_returns_ok() {
        let prefix = "bc1qq";

        let result = check_prefix(prefix.to_string());

        assert!(result.is_ok());
    }
}
