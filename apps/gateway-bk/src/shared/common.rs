use crate::error::{DakiaError, DakiaResult, ImmutStr};

// include!(concat!(env!("OUT_DIR"), "/ascii_version.rs"));
// include!(concat!(env!("OUT_DIR"), "/dakia_ascii_art.rs"));

static DAKIA_ASCII_ART: &str = "Welcome to Dakia Gateway";
static ASCII_VERSION: &str = "0.0.1";

pub fn exit() {
    std::process::exit(0);
}

pub fn get_dakia_ascii_art() -> String {
    DAKIA_ASCII_ART.to_string() + "\n\n" + get_ascii_version()
}

pub fn get_ascii_version() -> &'static str {
    ASCII_VERSION
}

pub fn get_dakia_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn _assert(cond: bool, msg: String) -> DakiaResult<()> {
    Ok(if !cond {
        return Err(DakiaError::i_explain(ImmutStr::Owned(msg.into_boxed_str())));
    })
}
