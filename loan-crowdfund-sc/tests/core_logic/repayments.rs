use loan_crowdfund_sc::{
    constants::ERR_INSUFFICIENT_REPAYMENT_AMOUNT,
    types::crowdfunding_state::{ProjectFundingState, INTEREST_RATE_DENOMINATION},
};

use crate::test_state::{
    mockups::{MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL, MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT},
    LoanCfTestState, INVESTOR_1_ADDRESS_EXPR,
};

#[test]
fn repayment_value_computed_correctly() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::Completed);

    state.check_total_repayment_amount(project_id, MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT);
}

#[test]
fn repayment_rate_computed_correctly() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::Completed);

    let expected_repayment_rate = MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT
        * INTEREST_RATE_DENOMINATION
        / MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL;
    state.check_repayment_rate(project_id, expected_repayment_rate);
}

#[test]
fn public_distribute_repayment() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::LoanRepaidNotComplete);

    state.public_distribute_repayment(INVESTOR_1_ADDRESS_EXPR, project_id);

    state.check_funding_state(project_id, ProjectFundingState::Completed);
}

#[test]
fn public_distribute_repayment_fails_when_repayment_is_below_treshold() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();

    let repayment_sc =
        state.create_default_mockup_in_state(project_id, &ProjectFundingState::LoanActive);
    state.repay_loan(&repayment_sc, 1);

    state.public_distribute_repayment_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        project_id,
        ERR_INSUFFICIENT_REPAYMENT_AMOUNT,
    );

    state.repay_loan(&repayment_sc, MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT);
    state.public_distribute_repayment(INVESTOR_1_ADDRESS_EXPR, project_id);
}

#[test]
fn admin_repayment_distribution_works_when_repayment_is_below_treshold() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();

    let repayment_sc =
        state.create_default_mockup_in_state(project_id, &ProjectFundingState::LoanActive);
    state.repay_loan(&repayment_sc, 1);

    state.admin_distribute_repayment(project_id);
}
