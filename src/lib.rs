// Specs
// Admin side
// - Create a loan: project id, crowdfunding start/end and target etc
// - cancel crowdfund and return funds (maybe)
// - approve crowdfund and proceed with funding
// - distribute repayments to users
// Bot side
// - send loan shares to users
// - distribute repayments to users
// User side
// - invest in a loan
// - withdraw from investing in a loan (MiCa compliance)
// - claim loan repayments
// - claim unsuccessful crowdfunding funds back

#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait LoanCrowdfundScContract {
    #[init]
    fn init(&self) {}
}
