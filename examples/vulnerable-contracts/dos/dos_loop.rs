use soroban_sdk::{Address, Env, Vec};

pub fn settle_all(e: Env, admin: Address, recipients: Vec<Address>) {
    admin.require_auth();

    for recipient in recipients.iter() {
        ledger::apply_reward(&e, recipient);
    }
}
