use loan_crowdfund_sc::types::crowdfunding_state::{
    ProjectFundingState, INTEREST_RATE_DENOMINATION,
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

    state.check_funding_state(project_id, ProjectFundingState::Completed);

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

    state.check_funding_state(project_id, ProjectFundingState::LoanRepaidNotComplete);

    state.public_distribute_repayment(INVESTOR_1_ADDRESS_EXPR, project_id);

    state.check_funding_state(project_id, ProjectFundingState::Completed);
}
