use loan_crowdfund_sc::constants::{
    ERR_CANNOT_WITHDRAW_IN_CRT_STATE, ERR_COOL_OFF_EXPIRED, ERR_INVESTMENT_NOT_FOUND,
};

use crate::test_state::{
    mockups::{MOCKUP_CF_TIMESTAMP_AFTER_END, MOCKUP_CF_TIMESTAMP_AFTER_START},
    LoanCfTestState, ACCOUNT_BALANCE_EXPR, INVESTOR_1_ADDRESS_EXPR,
};

#[test]
fn successful_cool_off_withdrawal() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.set_block_timestamp(102);

    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 1, 1000);

    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
    state.check_address_usdc_balance(INVESTOR_1_ADDRESS_EXPR, ACCOUNT_BALANCE_EXPR);
}

#[test]
fn successful_cool_off_withdrawal_with_multiple_investments() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.set_block_timestamp(102);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.set_block_timestamp(103);

    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 1, 1000);
    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 1, 1000);

    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
    state.check_address_usdc_balance(INVESTOR_1_ADDRESS_EXPR, ACCOUNT_BALANCE_EXPR);
}

#[test]
fn successful_cool_off_withdrawal_with_investment_in_multiple_projects() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.create_mocked_project_explicit_proj_id(2);
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 2);
    state.set_block_timestamp(102);

    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 1, 1000);
    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 2, 1000);

    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "2", "0");
    state.check_address_usdc_balance(INVESTOR_1_ADDRESS_EXPR, ACCOUNT_BALANCE_EXPR);
}

#[test]
fn failed_withdrawal_after_cool_off_state() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);

    state.withdraw_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        1,
        1000,
        ERR_CANNOT_WITHDRAW_IN_CRT_STATE,
    );
}

#[test]
fn failed_withdrawal_with_merged_investments() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START + 1);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
    state.set_block_timestamp(102);

    state.withdraw_and_expect_err(INVESTOR_1_ADDRESS_EXPR, 1, 2000, ERR_INVESTMENT_NOT_FOUND);
}
