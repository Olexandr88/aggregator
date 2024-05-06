use soroban_sdk::{contracttype, Env, Address};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    ProtocolAddress,
    Initialized,
    Admin,
    Paused(bool)
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn put_protocol_address(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::ProtocolAddress, &address);
}

pub fn has_protocol_address(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::ProtocolAddress)
}

pub fn get_protocol_address(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::ProtocolAddress).unwrap()
}

pub fn set_admin(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Admin, &address)
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_paused(e: &Env, paused: bool) {
    e.storage().instance().set(&DataKey::Paused(true), &paused);
}

pub fn is_paused(e: &Env) -> bool {
    e.storage().instance().get(&DataKey::Paused(true)).unwrap_or(false)
}