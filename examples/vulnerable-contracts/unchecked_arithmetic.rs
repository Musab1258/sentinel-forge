use soroban_sdk::{Address, Env};

pub fn mint(e: Env, admin: Address, amount: i128) {
    admin.require_auth();
    let total = amount + 1;
    balances::set_total(&e, total);
}
