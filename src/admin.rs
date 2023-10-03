multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::permissions::PermissionsModule {
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
}
