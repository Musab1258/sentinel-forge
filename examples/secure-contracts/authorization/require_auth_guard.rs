use soroban_sdk::{Address, Env};

pub fn execute_delegate(e: Env, admin: Address, delegate: Address, target_admin: Address) {
    admin.require_auth();
    assert!(delegate != target_admin);

    storage::remember_delegate(&e, &delegate);
    storage::set_admin(&e, &target_admin);
}
