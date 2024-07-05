#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, String};

mod event;
mod storage;
mod protocol_interface;
mod test;

use storage::{
    extend_instance_ttl, 
    set_initialized, 
    is_initialized, 
    set_protocol_id,
    get_protocol_id,
    set_protocol_address, 
    get_protocol_address, 
};
use soroswap_aggregator_adapter_interface::{SoroswapAggregatorAdapterTrait, AdapterError};
use protocol_interface::{protocol_swap_exact_tokens_for_tokens,
    protocol_swap_tokens_for_exact_tokens};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), AdapterError> {
    if amount < 0 {
        Err(AdapterError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), AdapterError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(AdapterError::DeadlineExpired)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), AdapterError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(AdapterError::NotInitialized)
    }
}

#[contract]
struct SoroswapAggregatorAdapter;

#[contractimpl]
impl SoroswapAggregatorAdapterTrait for SoroswapAggregatorAdapter {
    /// Initializes the contract and sets the phoenix multihop address
    fn initialize(
        e: Env,
        protocol_id: String,
        protocol_address: Address,
    ) -> Result<(), AdapterError> {
        if is_initialized(&e) {
            return Err(AdapterError::AlreadyInitialized);
        }
    
        set_protocol_id(&e, protocol_id.clone());
        set_protocol_address(&e, protocol_address.clone());
    
        set_initialized(&e);
        event::initialized(&e, true, protocol_id, protocol_address);
        extend_instance_ttl(&e);
        Ok(())
    }
    
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AdapterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        to.require_auth();

        check_nonnegative_amount(amount_in)?;
        check_nonnegative_amount(amount_out_min)?;
        ensure_deadline(&e, deadline)?;

        let swap_result = protocol_swap_exact_tokens_for_tokens(
            &e, 
            &amount_in, 
            &amount_out_min, 
            &path, 
            &to, 
            &deadline, 
        )?;

        event::swap(&e, amount_in, path, to);
        Ok(swap_result)
    }

    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AdapterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        to.require_auth();

        check_nonnegative_amount(amount_out)?;
        check_nonnegative_amount(amount_in_max)?;
        ensure_deadline(&e, deadline)?;

        let swap_result = protocol_swap_tokens_for_exact_tokens(
            &e, 
            &amount_out, 
            &amount_in_max, 
            &path, 
            &to, 
            &deadline, 
        )?;

        event::swap(&e, amount_in_max, path, to);
        Ok(swap_result)
    }

    /*  *** Read only functions: *** */
    fn get_protocol_id(e: &Env) -> Result<String, AdapterError> {
        check_initialized(&e)?;
        Ok(get_protocol_id(e)?)
    }    
    
    fn get_protocol_address(e: &Env) -> Result<Address, AdapterError> {
        check_initialized(&e)?;
        Ok(get_protocol_address(e)?)
    }    
}
