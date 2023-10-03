multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct UnlockSchedule<M: ManagedTypeApi> {
    pub project_id: u64,
    pub round: u64,
    pub amount: BigUint<M>,
    pub unlock_time: u64,
    pub is_claimed: bool,
}


// 01 - List<UnlockSchedule>