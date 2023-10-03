use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};
use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;
// thesis
/*
   Invalid = 0,
   Pending = 1, => prior starting
   CFActive = 2, => crowdfunding active - in between start and end timestamps, investment target not reached
   CFWaitingCooloff = 3, => crowdfunding active - in between start and end timestamps, investment target reached but partially in cool off
   CFSuccessful = 4, => crowdfunding active - in between start and end timestamps, investment target reached
   CFFailed = 5, => crowdfunding active - at end timestamp, investment target not reached
   CFCancelled = 6, => admin action
   LoanActive = 7, => withdrawn funds, awaiting repayment
   Completed = 8, => loan repayment completed, awaiting investor claim
*/

#[test]
fn funding_state_pending() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();

    state.check_funding_state(1, ProjectFundingState::Pending);
}

#[test]
fn funding_state_active() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("101");

    state.check_funding_state(1, ProjectFundingState::CFActive);
}

#[test]
fn funding_state_cool_off() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("101");
    state.invest(INVESTOR_1_ADDRESS_EXPR, 90001);

    state.check_funding_state(1, ProjectFundingState::CFWaitingCooloff);
}

#[test]
fn funding_state_successful() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("101");
    state.invest(INVESTOR_1_ADDRESS_EXPR, 90001);
    state.set_block_timestamp("10001");

    state.check_funding_state(1, ProjectFundingState::CFSuccessful);
}

#[test]
fn funding_state_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("10001");

    state.check_funding_state(1, ProjectFundingState::CFFailed);
}

#[test]
fn funding_state_cancelled() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.cancel_project(1);

    state.check_funding_state(1, ProjectFundingState::CFCancelled);
}

#[test]
fn funding_state_loan_active() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("101");
    state.invest(INVESTOR_1_ADDRESS_EXPR, 90001);
    state.set_block_timestamp("10001");

    state.claim_loan_funds(1);
    state.check_funding_state(1, ProjectFundingState::LoanActive);
}

#[test]
fn funding_state_completed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("101");
    state.invest(INVESTOR_1_ADDRESS_EXPR, 90001);
    state.set_block_timestamp("10001");
    state.claim_loan_funds(1);

    state.repay_loan(1, 11000);
    state.admin_distribute_repayment(1);

    state.check_funding_state(1, ProjectFundingState::Completed);
}

#[test]
fn funding_state_cool_off_to_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.set_block_timestamp("101");
    state.invest(INVESTOR_1_ADDRESS_EXPR, 90001);
    state.set_block_timestamp("102");

    state.check_funding_state(1, ProjectFundingState::CFWaitingCooloff);
    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 1, 90001);

    state.set_block_timestamp("10001");
    state.check_funding_state(1, ProjectFundingState::CFFailed);
}
