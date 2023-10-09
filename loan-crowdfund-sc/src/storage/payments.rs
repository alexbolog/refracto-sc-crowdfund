multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PaymentsModule {
    #[view(getRecordedPayments)]
    #[storage_mapper("recorded_payments")]
    fn recorded_payments(&self, project_id: u64) -> SetMapper<(u64, BigUint)>;
}
