pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), to]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
