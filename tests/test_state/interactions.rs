use loan_crowdfund_sc::{admin::ProxyTrait as _, beneficiary::ProxyTrait as _, ProxyTrait};
use multiversx_sc::types::Address;
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_buffer, managed_token_id,
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, SetStateStep},
    ContractInfo, ScenarioWorld,
};

use super::{
    world, LoanCfContract, LoanCfTestState, ACCOUNT_BALANCE_EXPR, BENEFICIARY_ADDRESS_EXPR,
    INVESTOR_1_ADDRESS_EXPR, INVESTOR_2_ADDRESS_EXPR, LOAN_CF_ADDRESS_EXPR, LOAN_SHARES_ID,
    OWNER_ADDRESS_EXPR, USDC_TOKEN_ID,
};

impl LoanCfTestState {
    pub fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS_EXPR, 1, LOAN_CF_ADDRESS_EXPR)
                .put_account(
                    BENEFICIARY_ADDRESS_EXPR,
                    Account::new().nonce(1).balance(ACCOUNT_BALANCE_EXPR),
                )
                .put_account(
                    INVESTOR_1_ADDRESS_EXPR,
                    Account::new().nonce(1).balance(ACCOUNT_BALANCE_EXPR),
                )
                .put_account(
                    INVESTOR_2_ADDRESS_EXPR,
                    Account::new().nonce(1).balance(ACCOUNT_BALANCE_EXPR),
                ),
        );

        let owner_address = AddressValue::from(OWNER_ADDRESS_EXPR).to_address();
        let beneficiary_address = AddressValue::from(BENEFICIARY_ADDRESS_EXPR).to_address();
        let investor_address_1 = AddressValue::from(INVESTOR_1_ADDRESS_EXPR).to_address();
        let investor_address_2 = AddressValue::from(INVESTOR_2_ADDRESS_EXPR).to_address();

        let contract = LoanCfContract::new(LOAN_CF_ADDRESS_EXPR);

        Self {
            world,
            contract,
            admin_address: owner_address,
            beneficiary_address,
            investor_address_1,
            investor_address_2,
        }
    }

    pub fn deploy_contract(&mut self) -> &mut Self {
        let code = self.world.code_expression(LOAN_CF_ADDRESS_EXPR);
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(code)
                .call(self.contract.init()),
        );

        self
    }

    pub fn invest(&mut self, investor_address_expr: &str, amount: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(investor_address_expr)
                .esdt_transfer(USDC_TOKEN_ID, 0, amount)
                .call(self.contract.invest()),
        );
    }

    pub fn withdraw(&mut self, investor_address_expr: &str, nonce: u64, amount: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(investor_address_expr)
                .esdt_transfer(LOAN_SHARES_ID, nonce, amount)
                .call(self.contract.withdraw()),
        );
    }

    pub fn claim(&mut self, investor_address_expr: &str, shares_nonce: u64, amount: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(investor_address_expr)
                .esdt_transfer(LOAN_SHARES_ID, shares_nonce, amount)
                .call(self.contract.claim()),
        );
    }

    pub fn public_distribute_repayment(&mut self, address_expr: &str) {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_expr)
                .call(self.contract.distribute_repayment()),
        );
    }

    pub fn create_project(
        &mut self,
        project_id: u64,
        project_name: &str,
        project_payment_token: &str,
        daily_interest_rate: u64,
        daily_penalty_fee_rate: u64,
        developer_wallet: Address,
        share_price_per_unit: u64,
        cf_start_timestamp: u64,
        cf_end_timestamp: u64,
        cf_target_min: u64,
        cf_target_max: u64,
    ) {
        self.world
            .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.contract.create_project(
                    project_id,
                    project_name,
                    managed_token_id!(project_payment_token),
                    daily_interest_rate,
                    daily_penalty_fee_rate,
                    developer_wallet,
                    share_price_per_unit,
                    cf_start_timestamp,
                    cf_end_timestamp,
                    cf_target_min,
                    cf_target_max,
                ),
            ));
    }

    pub fn cancel_project(&mut self, project_id: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .call(self.contract.cancel_project(project_id)),
        );
    }

    pub fn admin_distribute_repayment(&mut self, project_id: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .call(self.contract.admin_distribute_repayments(project_id)),
        );
    }

    pub fn claim_loan_funds(&mut self, project_id: u64) {
        self.world.sc_call(
            ScCallStep::new()
                .from(BENEFICIARY_ADDRESS_EXPR)
                .call(self.contract.claim_loan_funds(project_id)),
        );
    }

    pub fn repay_loan(&mut self, project_id: u64, amount: u64) {
        // self.world.sc_call(
        //     ScCallStep::new()
        //         .from(BENEFICIARY_ADDRESS_EXPR)
        //         .esdt_transfer(USDC_TOKEN_ID, 0, amount)
        //         .call(self.contract.repay_loan(project_id)),
        // );
    }
}
