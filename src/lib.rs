#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait LoanCrowdfundScContract {
    #[init]
    fn init(&self) {}
}
