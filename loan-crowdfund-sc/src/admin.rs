use crate::{constants::ERR_TOKEN_ISSUED, types::crowdfunding_state::CrowdfundingStateContext};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait AdminModule:
    crate::permissions::PermissionsModule
    + crate::storage::config::ConfigModule
    + crate::storage::payments::PaymentsModule
    + crate::common::CommonModule
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
        loan_start_timestamp: u64,
    ) {
        self.require_caller_is_admin();

        require!(
            self.crowdfunding_state(project_id).is_empty(),
            "PROJECT ALREADY EXISTS"
        );

        let escrow_sc_address =
            self.deploy_escrow_sc(project_id, &developer_wallet, &project_payment_token);

        let share_token_nonce =
            self.mint_project_shares(&cf_target_max, &share_price_per_unit, &project_name);

        let context = CrowdfundingStateContext::new(
            project_id,
            project_name,
            project_payment_token,
            daily_interest_rate,
            daily_penalty_fee_rate,
            developer_wallet,
            share_token_nonce,
            // 1,
            share_price_per_unit,
            cf_start_timestamp,
            cf_end_timestamp,
            cf_target_min,
            cf_target_max,
            loan_duration,
            loan_start_timestamp,
            escrow_sc_address,
        );

        self.crowdfunding_state(project_id).set(context);
        self.project_id_by_loan_share_nonce(share_token_nonce)
            .set(project_id);
    }

    #[endpoint(cancel)]
    fn cancel_project(&self, project_id: u64) {
        self.require_caller_is_admin();

        let mut context = self.crowdfunding_state(project_id).get();
        context.is_cancelled = true;
        self.crowdfunding_state(project_id).set(context);
    }

    #[endpoint(adminDistributeRepayment)]
    fn admin_distribute_repayments(&self, project_id: u64) {
        self.require_caller_is_admin();
        let mut cf_state = self.crowdfunding_state(project_id).get();
        self.process_payment_distribution(&mut cf_state, &BigUint::zero());
    }

    #[payable("*")]
    #[only_owner]
    #[endpoint(issueAndSetRoles)]
    fn issue_and_set_roles(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(
            self.loan_share_token_identifier().is_empty(),
            ERR_TOKEN_ISSUED
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

    fn deploy_escrow_sc(
        &self,
        project_id: u64,
        developer_wallet: &ManagedAddress,
        repayment_token_id: &TokenIdentifier,
    ) -> ManagedAddress {
        let code_metadata = CodeMetadata::all();
        let source_address = self.source_loan_repayment_sc_address().get();
        let mut args = ManagedArgBuffer::new();
        args.push_arg(project_id);
        args.push_arg(developer_wallet);
        args.push_arg(repayment_token_id);

        let (new_address, _) = self.send_raw().deploy_from_source_contract(
            self.blockchain().get_gas_left() - 1_500_000,
            &BigUint::zero(),
            &source_address,
            code_metadata,
            &args,
        );

        new_address
    }
}
