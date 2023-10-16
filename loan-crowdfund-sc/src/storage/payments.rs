multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PaymentsModule {
    #[view(getRecordedPayments)]
    #[storage_mapper("recorded_payments")]
    fn recorded_payments(&self, project_id: u64) -> SetMapper<(ManagedAddress, u64, BigUint)>;

    #[view(getRepaymentRate)]
    #[storage_mapper("repayment_rates")]
    fn repayment_rates(&self, project_id: u64) -> SingleValueMapper<BigUint>;
}
