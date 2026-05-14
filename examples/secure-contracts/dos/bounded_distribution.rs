use soroban_sdk::{Address, Env, Vec};

pub fn settle_batch(e: Env, admin: Address, recipients: Vec<Address>) {
    admin.require_auth();
    assert!(recipients.len() <= 32);

    for recipient in recipients.iter() {
        ledger::apply_reward(&e, recipient);
    }
}
