use clap::{App, Arg, ArgMatches};

pub const XPUB: &str = "xpub";
pub const PREFIX: &str = "prefix";

pub fn matches() -> ArgMatches<'static> {
    App::new("senzu")
        .version("0.1.0")
        .about("Vanity address generator searching only addresses derived from your xpub")
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
                .multiple(true),
        )
        .get_matches()
}

// TODO: Check xpub is syntactically valid
// TODO: Check prefix has valid chars, is not too long, and starts with bc1q
