/// Afnan Enayet 2017
///
/// `rustchain` - a blockchain writte in Rust

pub mod block;
pub mod blockchain;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
