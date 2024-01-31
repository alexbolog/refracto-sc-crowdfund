use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;

use crate::test_state::{
    mockups::{
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL, MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT,
        MOCKUP_CF_SMALL_INVESTMENT,
    },
    LoanCfTestState, ACCOUNT_BALANCE_EXPR, INVESTOR_1_ADDRESS_EXPR,
};

#[ignore = "Fails because of denomination"]
#[test]
fn claim_rewards_cancelled_project() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::CFCancelled);

    state.claim_refund(INVESTOR_1_ADDRESS_EXPR, 1, MOCKUP_CF_SMALL_INVESTMENT);

    state.check_address_usdc_balance(INVESTOR_1_ADDRESS_EXPR, ACCOUNT_BALANCE_EXPR);
}

#[test]
fn claim_rewards_completed_project() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::Completed);

    state.claim(
        INVESTOR_1_ADDRESS_EXPR,
        1,
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
    );

    let initial_balance: u64 = ACCOUNT_BALANCE_EXPR
        .parse()
        .expect("Failed to parse string to u64");
    let expected_balance = initial_balance - MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL
        + MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT;

    state.check_address_usdc_balance(INVESTOR_1_ADDRESS_EXPR, &expected_balance.to_string());
}
