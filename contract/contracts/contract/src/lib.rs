#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map};

#[contract]
pub struct CrowdContract;

#[contractimpl]
impl CrowdContract {

    // Create project
    pub fn create(env: Env, id: Symbol, goal: i128) {
        let mut projects: Map<Symbol, i128> =
            env.storage().instance().get(&Symbol::short("P")).unwrap_or(Map::new(&env));

        projects.set(id, goal);

        env.storage().instance().set(&Symbol::short("P"), &projects);
    }

    // Fund project
    pub fn fund(env: Env, id: Symbol, amount: i128) {
        let mut projects: Map<Symbol, i128> =
            env.storage().instance().get(&Symbol::short("P")).unwrap();

        let current = projects.get(id.clone()).unwrap_or(0);

        projects.set(id, current + amount);

        env.storage().instance().set(&Symbol::short("P"), &projects);
    }

    // View project
    pub fn get(env: Env, id: Symbol) -> i128 {
        let projects: Map<Symbol, i128> =
            env.storage().instance().get(&Symbol::short("P")).unwrap();

        projects.get(id).unwrap_or(0)
    }
}