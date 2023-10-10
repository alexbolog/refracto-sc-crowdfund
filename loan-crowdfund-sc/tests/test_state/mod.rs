use loan_crowdfund_sc::{admin::ProxyTrait as _, beneficiary::ProxyTrait as _, ProxyTrait};
use multiversx_sc::types::Address;
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_buffer, managed_token_id,
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, SetStateStep},
    ContractInfo, ScenarioWorld,
};

pub const LOAN_CF_ADDRESS_EXPR: &str = "sc:loan-cf-address";
pub const OWNER_ADDRESS_EXPR: &str = "address:owner";
pub const BENEFICIARY_ADDRESS_EXPR: &str = "address:beneficiary";
pub const INVESTOR_1_ADDRESS_EXPR: &str = "address:investor1";
pub const INVESTOR_2_ADDRESS_EXPR: &str = "address:investor2";
pub const ACCOUNT_BALANCE_EXPR: &str = "100,000,000";

pub const USDC_TOKEN_ID_EXPR: &str = "str:USDC-123456";
pub const USDC_TOKEN_ID: &str = "USDC-123456";
pub const LOAN_SHARES_ID_EXPR: &str = "str:REFRACTO-123456";
pub const LOAN_SHARES_ID: &str = "REFRACTO-123456";
pub const INVALID_TOKEN_ID_EXPR: &str = "str:RANDOMTKN-123456";

pub mod bc_state_interactions;
pub mod checks;
pub mod interactions;
pub mod mockups;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("../");

    blockchain.register_contract(LOAN_CF_ADDRESS_EXPR, loan_crowdfund_sc::ContractBuilder);

    blockchain
}

type LoanCfContract = ContractInfo<loan_crowdfund_sc::Proxy<StaticApi>>;

pub struct LoanCfTestState {
    pub world: ScenarioWorld,
    pub contract: LoanCfContract,
    pub admin_address: Address,
    pub beneficiary_address: Address,
    pub investor_address_1: Address,
    pub investor_address_2: Address,
}
