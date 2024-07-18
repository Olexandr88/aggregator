#![cfg(test)]
extern crate std;
use crate::models::Adapter;
use crate::{SoroswapAggregator, SoroswapAggregatorClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    vec, Address, BytesN, Env, String, Vec,
};

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../adapters/soroswap/soroswap_contracts/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

// // Pair Contract
// mod pair {
//     soroban_sdk::contractimport!(file = "../../protocols/soroswap/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
//     pub type SoroswapPairClient<'a> = Client<'a>;
// }
// use pair::SoroswapPairClient;

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../adapters/soroswap/soroswap_contracts/soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../adapters/soroswap/soroswap_contracts/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
mod router {
    soroban_sdk::contractimport!(file = "../adapters/soroswap/soroswap_contracts/soroswap_router.optimized.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}
use router::SoroswapRouterClient;

// SoroswapRouter Contract
pub fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    let router_address = &e.register_contract_wasm(None, router::WASM);
    let router = SoroswapRouterClient::new(e, router_address);
    router
}
// SoroswapAggregatorAdapter Contract
// For Soroswap
mod soroswap_adapter {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/soroswap_adapter.optimized.wasm");
    pub type SoroswapAggregatorAdapterForSoroswapClient<'a> = Client<'a>;
}
use soroswap_adapter::SoroswapAggregatorAdapterForSoroswapClient;

// Adapter for Soroswap
fn create_soroswap_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterForSoroswapClient<'a> {
    let adapter_address = &e.register_contract_wasm(None, soroswap_adapter::WASM);
    let adapter = SoroswapAggregatorAdapterForSoroswapClient::new(e, adapter_address);
    adapter
}

// SoroswapAggregatorAdapter Contract
// For Phoenix
mod phoenix_adapter {
    soroban_sdk::contractimport!(
        file =
            "../target/wasm32-unknown-unknown/release/phoenix_adapter.optimized.wasm"
    );
    pub type SoroswapAggregatorAdapterForPhoenixClient<'a> = Client<'a>;
}
use phoenix_adapter::SoroswapAggregatorAdapterForPhoenixClient;

// Adapter for phoenix
fn create_phoenix_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterForPhoenixClient<'a> {
    let adapter_address = &e.register_contract_wasm(None, phoenix_adapter::WASM);
    let adapter = SoroswapAggregatorAdapterForPhoenixClient::new(e, adapter_address);
    adapter
}

// SoroswapAggregator Contract
fn create_soroswap_aggregator<'a>(e: &Env) -> SoroswapAggregatorClient<'a> {
    SoroswapAggregatorClient::new(e, &e.register_contract(None, SoroswapAggregator {}))
}

// Helper function to initialize / update soroswap aggregator protocols
pub fn create_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
    vec![
        &test.env,
        Adapter {
            protocol_id: String::from_str(&test.env, "soroswap"),
            address: test.soroswap_adapter_contract.address.clone(),
            paused: false,
        },
    ]
}

pub fn new_update_adapters_addresses(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
    vec![
        &test.env,
        Adapter {
            protocol_id: String::from_str(&test.env, "some_protocol"),
            address: test.router_contract.address.clone(),
            paused: false,
        },
    ]
}

// pub fn create_only_soroswap_protocol_address(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
//     vec![&test.env,
//         Adapter {
//             protocol_id: dex_constants::SOROSWAP,
//             address: test.router_contract.address.clone(),
//         },
//     ]
// }

// pub fn create_only_phoenix_protocol_address(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
//     vec![&test.env,
//         Adapter {
//             protocol_id: dex_constants::PHOENIX,
//             address: test.router_contract.address.clone(),
//         },
//     ]
// }

pub struct SoroswapAggregatorTest<'a> {
    env: Env,
    aggregator_contract: SoroswapAggregatorClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    // factory_contract: SoroswapFactoryClient<'a>,
    soroswap_adapter_contract: SoroswapAggregatorAdapterForSoroswapClient<'a>,
    // phoenix_adapter_contract: SoroswapAggregatorAdapterForPhoenixClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    token_2: TokenClient<'a>,
    user: Address,
    admin: Address,
}

impl<'a> SoroswapAggregatorTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let aggregator_contract = create_soroswap_aggregator(&env);
        let router_contract = create_soroswap_router(&env);
        let soroswap_adapter_contract = create_soroswap_adapter(&env);
        let _phoenix_adapter_contract = create_phoenix_adapter(&env);

        let initial_user_balance = 20_000_000_000_000_000_000;

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let token_0 = create_token_contract(&env, &admin);
        let token_1 = create_token_contract(&env, &admin);
        let token_2 = create_token_contract(&env, &admin);

        token_0.mint(&user, &initial_user_balance);
        token_1.mint(&user, &initial_user_balance);
        token_2.mint(&user, &initial_user_balance);

        let factory_contract = create_soroswap_factory(&env, &admin);
        env.budget().reset_unlimited();

        let ledger_timestamp = 100;
        let desired_deadline = 1000;

        assert!(desired_deadline > ledger_timestamp);

        env.ledger().with_mut(|li| {
            li.timestamp = ledger_timestamp;
        });

        let amount_0: i128 = 1_000_000_000_000_000_000;
        let amount_1: i128 = 4_000_000_000_000_000_000;
        let amount_2: i128 = 8_000_000_000_000_000_000;
        let expected_liquidity: i128 = 2_000_000_000_000_000_000;

        // Check initial user value of every token:
        assert_eq!(token_0.balance(&user), initial_user_balance);
        assert_eq!(token_1.balance(&user), initial_user_balance);
        assert_eq!(token_2.balance(&user), initial_user_balance);

        router_contract.initialize(&factory_contract.address);

        assert_eq!(
            factory_contract.pair_exists(&token_0.address, &token_1.address),
            false
        );
        let (added_token_0_0, added_token_1_0, added_liquidity_0_1) = router_contract
            .add_liquidity(
                &token_0.address,  //     token_a: Address,
                &token_1.address,  //     token_b: Address,
                &amount_0,         //     amount_a_desired: i128,
                &amount_1,         //     amount_b_desired: i128,
                &0,                //     amount_a_min: i128,
                &0,                //     amount_b_min: i128,
                &user,             //     to: Address,
                &desired_deadline, //     deadline: u64,
            );

        let (added_token_1_1, added_token_2_0, added_liquidity_1_2) = router_contract
            .add_liquidity(
                &token_1.address,  //     token_a: Address,
                &token_2.address,  //     token_b: Address,
                &amount_1,         //     amount_a_desired: i128,
                &amount_2,         //     amount_b_desired: i128,
                &0,                //     amount_a_min: i128,
                &0,                //     amount_b_min: i128,
                &user,             //     to: Address,
                &desired_deadline, //     deadline: u64,
            );

        // let (added_token_0_1, added_token_2_1, added_liquidity_0_2) = router_contract.add_liquidity(
        //     &token_0.address, //     token_a: Address,
        //     &token_2.address, //     token_b: Address,
        //     &amount_0, //     amount_a_desired: i128,
        //     &amount_1, //     amount_b_desired: i128,
        //     &0, //     amount_a_min: i128,
        //     &0 , //     amount_b_min: i128,
        //     &user, //     to: Address,
        //     &desired_deadline//     deadline: u64,
        // );

        static MINIMUM_LIQUIDITY: i128 = 1000;

        assert_eq!(added_token_0_0, amount_0);
        assert_eq!(added_token_1_0, amount_1);
        assert_eq!(added_token_1_1, amount_1);
        assert_eq!(added_token_2_0, amount_2);
        // assert_eq!(added_token_0_1, amount_0);
        // assert_eq!(added_token_2_1, amount_1);

        assert_eq!(
            added_liquidity_0_1,
            expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap()
        );
        assert_eq!(added_liquidity_1_2, 5656854249492379195);
        // assert_eq!(added_liquidity_0_2, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());

        assert_eq!(token_0.balance(&user), 19_000_000_000_000_000_000);
        assert_eq!(token_1.balance(&user), 12_000_000_000_000_000_000);
        assert_eq!(token_2.balance(&user), 12_000_000_000_000_000_000);

        // Initializing Soroswap Adapter Contract
        soroswap_adapter_contract.initialize(
            &String::from_str(&env, "soroswap"),
            &router_contract.address,
        );

        SoroswapAggregatorTest {
            env,
            aggregator_contract,
            router_contract,
            // factory_contract,
            soroswap_adapter_contract,
            // phoenix_adapter_contract,
            token_0,
            token_1,
            token_2,
            user,
            admin,
        }
    }
}

pub mod events;
pub mod get_adapters;
pub mod initialize;
pub mod remove_adapter;
pub mod set_pause_get_paused;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;
pub mod update_adapters;
// pub mod swap;
// pub mod admin;
