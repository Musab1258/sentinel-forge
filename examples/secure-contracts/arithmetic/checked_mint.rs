use soroban_sdk::{Address, Env};

pub fn increase_supply(e: Env, admin: Address, amount: i128, bonus: i128) {
    admin.require_auth();
    let minted = amount.checked_add(bonus).expect("overflow");
    balances::set_total(&e, minted);
}
