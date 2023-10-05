use test_state::LoanCfTestState;

mod core_logic;
mod invest;
mod project_funding_state;
mod test_state;

#[test]
fn test_deploy() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
}
