use loan_crowdfund_sc::{
    constants::{ERR_CANNOT_INVEST_IN_CRT_STATE, ERR_CANNOT_OVER_FINANCE},
    types::crowdfunding_state::ProjectFundingState,
};

use crate::test_state::{
    mockups::{MOCKUP_CF_DEFAULT_MAX_PRINCIPAL, MOCKUP_CF_DEFAULT_MIN_PRINCIPAL},
    LoanCfTestState, INVESTOR_1_ADDRESS_EXPR, INVESTOR_2_ADDRESS_EXPR,
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
    let initial_investment = MOCKUP_CF_DEFAULT_MIN_PRINCIPAL - 1;
    let over_financing_investment = MOCKUP_CF_DEFAULT_MAX_PRINCIPAL - initial_investment + 1;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(1, &ProjectFundingState::CFActive);
    state.whitelist_address(INVESTOR_2_ADDRESS_EXPR);

    state.invest(INVESTOR_1_ADDRESS_EXPR, initial_investment, 1);
    state.invest_and_expect_err(
        INVESTOR_2_ADDRESS_EXPR,
        over_financing_investment,
        1,
        ERR_CANNOT_OVER_FINANCE,
    );

    state.invest(
        INVESTOR_1_ADDRESS_EXPR,
        MOCKUP_CF_DEFAULT_MAX_PRINCIPAL - initial_investment,
        1,
    );
}
