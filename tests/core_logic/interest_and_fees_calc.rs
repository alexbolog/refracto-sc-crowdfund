use loan_crowdfund_sc::types::crowdfunding_state::INTEREST_RATE_DENOMINATION;

use crate::test_state::LoanCfTestState;

#[test]
fn interest_applied_correctly() {
    let project_id = 1;
    let daily_interest_rate = 100;
    let days = 10;
    let total_principal = 100_000 * INTEREST_RATE_DENOMINATION;

    let expected_interest = days * daily_interest_rate;
    let expected_total = total_principal + expected_interest;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_mocked_project_explicit_financing_details(
        project_id,
        total_principal,
        total_principal,
        daily_interest_rate,
        0,
    );

    state.set_block_timestamp("101");
    state.invest("investor_1", total_principal, project_id);

    let target_timestamp = 101 + days * 24 * 3600;
    state.set_block_timestamp(&target_timestamp.to_string());

    state.check_expected_interest(project_id, expected_interest);
    state.check_total_repayment_amount(project_id, expected_total);
}
