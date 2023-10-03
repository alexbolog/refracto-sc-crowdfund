// Specs
// Admin side
// - Create a loan: project id, crowdfunding start/end and target etc
// - cancel crowdfund and return funds (maybe)
// - approve crowdfund and proceed with funding -> might not be needed, if we make the developer claim the funds
// - distribute repayments to users

// Bot side
// - send loan shares to users
// - distribute repayments to users

// User side
// - invest in a loan
// - withdraw from investing in a loan (MiCa compliance)
// - claim loan repayments
// - claim unsuccessful crowdfunding funds back

// Beneficiary side (developer)
// - claim funds from successful crowdfunding
// - repay loan
// - interest rate calculator + penalty fee calculator

// Add expected repayment amount check in repayment distribution

// !!! Single, full claiming + interest

#![no_std]

multiversx_sc::imports!();

mod kyc;
mod permissions;
mod storage;
pub mod types;

#[multiversx_sc::contract]
pub trait LoanCrowdfundScContract:
    permissions::PermissionsModule + storage::config::ConfigModule + kyc::KycModule
{
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint(invest)]
    fn invest(&self) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_kyc_compliant(&caller);
    }

    fn mint_loan_shares(&self) -> EsdtTokenPayment {
        todo!()
    }

    fn update_cf_progress(&self) {}
}
