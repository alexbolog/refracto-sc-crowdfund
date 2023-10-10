use loan_crowdfund_sc::types::crowdfunding_state::INTEREST_RATE_DENOMINATION;

use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};

// interest = 14%
// principal = 100k

// daily interest = 14% / 365 = 0.0383561643835616% / 100 = 0.000383561643835616
// dobanda dupa 10 zile = 10 * daily_interest * principal = 10 * 0.000383561643835616 * 100k = 383.561643835616
// dobanda dupa 365 zile = 365 * daily_interest * principal = 365 * 0.000383561643835616 * 100k = 14000

const ONE_YEAR: u64 = 12 * 30 * 24 * 3600;

#[test]
fn correct_interest_calculation() {
    let project_id = 1;
    let apr = 14;
    let daily_interest_rate = INTEREST_RATE_DENOMINATION * apr / 365_00; // 365 days * 100

    let days = 10;
    let total_principal = 100_000;

    let expected_interest =
        days * daily_interest_rate * total_principal / INTEREST_RATE_DENOMINATION;
    let target_timestamp = 101 + days * 24 * 3600;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.create_mocked_project_explicit_financing_details(
        project_id,
        total_principal,
        total_principal,
        daily_interest_rate,
        0,
        ONE_YEAR,
    );

    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, total_principal, project_id);
    state.claim_loan_funds(project_id);

    state.set_block_timestamp(target_timestamp);

    state.check_expected_interest(project_id, expected_interest);
}

#[test]
fn correct_late_fees_calculation() {
    let project_id = 1;
    let penalty_apr = 5;
    let daily_penalty_rate = INTEREST_RATE_DENOMINATION * penalty_apr / 365_00; // 365 days * 100

    let days = 10;
    let total_principal = 100_000;

    let expected_fees = days * daily_penalty_rate * total_principal / INTEREST_RATE_DENOMINATION;
    let target_timestamp = 101 + ONE_YEAR + days * 24 * 3600;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);
    state.create_mocked_project_explicit_financing_details(
        project_id,
        total_principal,
        total_principal,
        0,
        daily_penalty_rate,
        ONE_YEAR,
    );
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, total_principal, project_id);
    state.claim_loan_funds(project_id);

    state.set_block_timestamp(target_timestamp);

    state.check_expected_late_fees(project_id, expected_fees);
}

#[test]
fn interest_and_late_fees_applied_correctly() {
    let project_id = 1;
    let apr = 14;
    let daily_interest_rate = INTEREST_RATE_DENOMINATION * apr / 365;
    let late_fees = 1;
    let principal = 100_000;
    let days_late = 10;
    let loan_duration = 10 * 24 * 3600; // 10 days

    let late_fees_per_day_rate = late_fees * INTEREST_RATE_DENOMINATION / 365;
    let expected_interest =
        principal * daily_interest_rate * days_late / INTEREST_RATE_DENOMINATION;
    let expected_late_fees =
        late_fees_per_day_rate * days_late * principal / INTEREST_RATE_DENOMINATION;
    let expected_total_amount = principal + expected_interest + expected_late_fees;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_mocked_project_explicit_financing_details(
        project_id,
        principal,
        principal,
        daily_interest_rate,
        late_fees_per_day_rate,
        loan_duration,
    );
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, principal, project_id);
    state.set_block_timestamp(101 + loan_duration + expected_late_fees);

    state.check_expected_interest(project_id, expected_interest);
    state.check_expected_late_fees(project_id, expected_late_fees);
    state.check_total_repayment_amount(project_id, expected_total_amount);
}
