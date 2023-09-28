multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct UnlockSchedule<M: ManagedTypeApi> {
    pub project_id: u64,
    pub amount: BigUint<M>,
    pub unlock_time: u64,
}
