mod ctx;
mod helpers;
mod proxy;
mod session;
mod load_balancer;

use std::collections::HashMap;

pub use proxy::Proxy;
pub use session::Session;


pub type HeaderBuffer = HashMap<String, Vec<u8>>;