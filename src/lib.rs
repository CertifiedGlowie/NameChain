#![allow(clippy::upper_case_acronyms)]
use std::net::{Ipv4Addr, Ipv6Addr};

mod chain;
mod domains;
mod transactions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
