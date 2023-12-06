use loan_crowdfund_sc::constants::COOL_OFF_PERIOD;
use multiversx_sc_scenario::scenario_model::SetStateStep;
use super::LoanCfTestState;

impl LoanCfTestState {
    pub fn set_block_timestamp(&mut self, block_timestamp_expr: u64) {
        self.world.set_state_step(
            SetStateStep::new().block_timestamp(COOL_OFF_PERIOD + block_timestamp_expr),
        );
    }
}
