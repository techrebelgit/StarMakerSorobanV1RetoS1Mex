#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const RESULTADO: Symbol = symbol_short!("RESULTADO");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn sumar(env: Env, a: i128, b: i128) -> i128 {
        let result = a + b;
        env.storage().persistent().set(&RESULTADO, &result); // ✅ REQUIRED for 22.0.8
        result
    }

    pub fn resultado_anterior(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get::<Symbol, i128>(&RESULTADO)
            .unwrap_or(0)
    }
}

mod test;
