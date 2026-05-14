use soroban_sdk::{Address, Env};

pub fn execute_delegate(e: Env, delegate: Address, target_admin: Address) {
    storage::remember_delegate(&e, &delegate);
    storage::set_admin(&e, &target_admin);
}
