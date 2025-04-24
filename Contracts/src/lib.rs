#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env) -> &'static str {
        "Hello, Stellar!"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::EnvExt;

    #[test]
    fn test_hello() {
        let env = Env::default();
        assert_eq!(HelloContract::hello(env), "Hello, Stellar!");
    }
}
