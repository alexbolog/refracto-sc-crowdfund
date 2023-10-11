use loan_crowdfund_sc::{
    constants::{ERR_CANNOT_INVEST_IN_CRT_STATE, ERR_CANNOT_OVER_FINANCE},
    types::crowdfunding_state::ProjectFundingState,
};

use crate::test_state::{
    mockups::MOCKUP_CF_TIMESTAMP_AFTER_START, LoanCfTestState, INVESTOR_1_ADDRESS_EXPR,
};

#[test]
fn investing_is_only_allowed_with_cf_active() {
    let project_funding_states = [
        // ProjectFundingState::Invalid,
        ProjectFundingState::Pending,
        ProjectFundingState::CFWaitingCooloff,
        ProjectFundingState::CFSuccessful,
        ProjectFundingState::CFFailed,
        ProjectFundingState::CFCancelled,
        ProjectFundingState::LoanActive,
        // TODO: Uncomment when completed state is implemented in mockups
        // ProjectFundingState::Completed,
    ];

    for funding_state in project_funding_states.iter() {
        let mut state = LoanCfTestState::new();
        state.deploy_contract();
        state.create_default_mockup_in_state(1, &funding_state);
        state.check_funding_state(1, funding_state.clone());
        state.invest_and_expect_err(
            INVESTOR_1_ADDRESS_EXPR,
            100,
            1,
            ERR_CANNOT_INVEST_IN_CRT_STATE,
        );
    }
}

#[test]
fn failed_over_financing() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 99999, 1);

    state.invest_and_expect_err(INVESTOR_1_ADDRESS_EXPR, 100, 1, ERR_CANNOT_OVER_FINANCE)
}

#[test]
fn test_mockup_maker() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(1, &ProjectFundingState::CFSuccessful);
    state.check_funding_state(1, ProjectFundingState::CFSuccessful);
}
