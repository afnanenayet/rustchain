/// Afnan Enayet 2017
///
/// `rustchain` - a blockchain written in Rust

pub mod block;
pub mod blockchain;
pub mod transaction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
