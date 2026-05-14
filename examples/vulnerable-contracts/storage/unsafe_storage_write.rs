use soroban_sdk::{Address, Env};

pub fn overwrite_recipient(e: Env, recipient: Address) {
    storage::set_recipient(&e, &recipient);
}
