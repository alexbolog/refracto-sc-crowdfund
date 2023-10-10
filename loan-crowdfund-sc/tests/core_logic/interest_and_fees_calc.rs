use loan_crowdfund_sc::types::crowdfunding_state::INTEREST_RATE_DENOMINATION;

use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};

// interest = 14%
// principal = 100k

// daily interest = 14% / 365 = 0.0383561643835616% / 100 = 0.000383561643835616
// dobanda dupa 10 zile = 10 * daily_interest * principal
// dobanda dupa 365 zile = 365 * daily_interest * principal = 365 * 0.000383561643835616 * 100k = 14000

#[test]
fn interest_applied_correctly() {
    let project_id = 1;
    let apr = 14;
    let daily_interest_rate = INTEREST_RATE_DENOMINATION * apr / 365;

    let days = 10;
    let total_principal = 100_000;

    let expected_interest =
        total_principal * daily_interest_rate * days / INTEREST_RATE_DENOMINATION;
    let expected_total = total_principal + expected_interest;
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
        12 * 30 * 24 * 3600,
    );

    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, total_principal, project_id);

    state.set_block_timestamp(target_timestamp);

    state.check_expected_interest(project_id, expected_interest);
    state.check_total_repayment_amount(project_id, expected_total);
}

#[test]
fn late_fees_applied_correctly() {
    let project_id = 1;
    let late_fees = 1;
    let principal = 100_000;
    let days_late = 10;
    let loan_duration = 10 * 24 * 3600; // 10 days

    let late_fees_per_day_rate = late_fees * INTEREST_RATE_DENOMINATION / 365;
    let expected_late_fees =
        late_fees_per_day_rate * days_late * principal / INTEREST_RATE_DENOMINATION;
    let expected_total_amount = principal + expected_late_fees;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_mocked_project_explicit_financing_details(
        project_id,
        principal,
        principal,
        0,
        late_fees_per_day_rate,
        loan_duration,
    );
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, principal, project_id);
    state.set_block_timestamp(101 + loan_duration + expected_late_fees);

    state.check_expected_late_fees(project_id, expected_late_fees);
    state.check_total_repayment_amount(project_id, expected_total_amount);
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
