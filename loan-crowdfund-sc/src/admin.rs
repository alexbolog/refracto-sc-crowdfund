multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait AdminModule:
    crate::permissions::PermissionsModule + crate::storage::config::ConfigModule
{
    #[endpoint(create)]
    fn create_project(
        &self,
        project_id: u64,
        project_name: ManagedBuffer,
        project_payment_token: TokenIdentifier,
        daily_interest_rate: u64,
        daily_penalty_fee_rate: u64,
        developer_wallet: ManagedAddress,
        share_price_per_unit: BigUint,
        cf_start_timestamp: u64,
        cf_end_timestamp: u64,
        cf_target_min: BigUint,
        cf_target_max: BigUint,
        loan_duration: u64,
    ) {
    }

    #[endpoint(cancel)]
    fn cancel_project(&self, project_id: u64) {}

    #[endpoint(adminDistributeRepayment)]
    fn admin_distribute_repayments(&self, project_id: u64) {}

    #[payable("*")]
    #[only_owner]
    #[endpoint(issueAndSetRoles)]
    fn issue_and_set_roles(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(
            self.loan_share_token_identifier().is_empty(),
            "TOKEN ALREADY ISSUED"
        );
        let issue_cost = self.call_value().egld_value().clone_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_and_set_all_roles(
                issue_cost,
                token_name,
                token_ticker,
                EsdtTokenType::Meta,
                18,
            )
            .async_call()
            .with_callback(self.callbacks().issue_and_set_roles_callback())
            .call_and_exit()
    }

    #[callback]
    fn issue_and_set_roles_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.loan_share_token_identifier()
                    .set(token_identifier.as_esdt_option().unwrap());
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }
}
