use soroban_sdk::{Env, Address, Vec, symbol_short};

pub fn is_paused(e: &Env) -> bool {
    e.storage().instance().get(&symbol_short!("PAUSED")).unwrap_or(false)
}

pub fn set_paused(e: &Env, val: bool) {
    e.storage().instance().set(&symbol_short!("PAUSED"), &val);
}

pub fn guardians(e: &Env) -> Vec<Address> {
    e.storage().instance().get(&symbol_short!("GUARDS")).unwrap_or(Vec::new(e))
}

pub fn require_guardian(e: &Env, caller: Address) {
    let guards = guardians(e);

    if !guards.contains(&caller) {
        panic!("Not authorized guardian");
    }
}

pub fn pause(e: &Env, caller: Address) {
    require_guardian(e, caller.clone());

    set_paused(e, true);

    e.events().publish(
        (symbol_short!("PAUSE"),),
        caller,
    );
}

pub fn unpause(e: &Env, caller: Address) {
    require_guardian(e, caller.clone());

    set_paused(e, false);

    e.events().publish(
        (symbol_short!("UNPAUSE"),),
        caller,
    );
}

use crate::pause::*;

pub fn create_policy(env: Env, user: Address) {
    if is_paused(&env) {
        panic!("Contract paused");
    }
}

use crate::pause::*;

pub fn submit_claim(env: Env, user: Address) {
    if is_paused(&env) {
        panic!("Claims paused");
    }pub fn add_guardian(env: Env, guardian: Address) {
    let mut guards = guardians(&env);
    guards.push_back(guardian);
    env.storage().instance().set(&symbol_short!("GUARDS"), &guards);
}

  pub fn get_pause_status(env: Env) -> bool {
    is_paused(&env)
}
}

