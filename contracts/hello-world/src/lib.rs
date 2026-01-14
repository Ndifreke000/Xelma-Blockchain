#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, vec, Env};

    #[test]
    fn test_hello() {
        let env = Env::default();
        let contract_id = env.register(HelloWorld, ());
        let client = HelloContractClient::new(&env, &contract_id);

        let words = client.hello(&symbol_short!("World"));
        assert_eq!(
            words,
            vec![&env, symbol_short!("Hello"), symbol_short!("World"),]
        );
    }
}
