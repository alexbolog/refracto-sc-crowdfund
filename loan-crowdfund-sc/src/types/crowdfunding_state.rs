multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const INTEREST_RATE_DENOMINATION: u64 = 1_000_000_000;

#[derive(TopEncode, TopDecode, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode)]
pub struct CrowdfundingStateContext<M: ManagedTypeApi> {
    pub project_id: u64,
    pub project_name: ManagedBuffer<M>, // maybe?
    pub project_payment_token: TokenIdentifier<M>,

    pub daily_interest_rate: u64, // dobanda zilnica
    pub daily_penalty_fee_rate: u64,
    pub developer_wallet: ManagedAddress<M>,

    pub share_token_nonce: u64,
    pub share_price_per_unit: BigUint<M>,

    pub cf_start_timestamp: u64,
    pub cf_end_timestamp: u64,

    pub cf_target_min: BigUint<M>,
    pub cf_target_max: BigUint<M>,
    pub cf_progress: BigUint<M>,

    pub loan_duration: u64,
    pub loan_start_timestamp: u64,
    pub repayment_contract_address: ManagedAddress<M>,

    pub is_cancelled: bool,
    pub is_loan_active: bool,
    pub is_repayed: bool,
}

impl<M: ManagedTypeApi> CrowdfundingStateContext<M> {
    pub fn new(
        project_id: u64,
        project_name: ManagedBuffer<M>,
        project_payment_token: TokenIdentifier<M>,
        daily_interest_rate: u64,
        daily_penalty_fee_rate: u64,
        developer_wallet: ManagedAddress<M>,
        share_token_nonce: u64,
        share_price_per_unit: BigUint<M>,
        cf_start_timestamp: u64,
        cf_end_timestamp: u64,
        cf_target_min: BigUint<M>,
        cf_target_max: BigUint<M>,
        loan_duration: u64,
        repayment_contract_address: ManagedAddress<M>,
    ) -> Self {
        CrowdfundingStateContext {
            project_id,
            project_name,
            project_payment_token,
            daily_interest_rate,
            daily_penalty_fee_rate,
            developer_wallet,
            share_token_nonce,
            share_price_per_unit,
            cf_start_timestamp,
            cf_end_timestamp,
            cf_target_min,
            cf_target_max,
            cf_progress: BigUint::zero(),
            loan_duration,
            loan_start_timestamp: 0,
            repayment_contract_address,
            is_cancelled: false,
            is_loan_active: false,
            is_repayed: false,
        }
    }

    pub fn get_funding_state(
        &self,
        amount_cooling_off: &BigUint<M>,
        block_timestamp: u64,
    ) -> ProjectFundingState {
        if self.is_cancelled {
            return ProjectFundingState::CFCancelled;
        }

        if self.is_loan_active {
            return ProjectFundingState::LoanActive;
        }

        if self.is_repayed {
            return ProjectFundingState::Completed;
        }

        if block_timestamp < self.cf_start_timestamp {
            return ProjectFundingState::Pending;
        }

        if block_timestamp >= self.cf_start_timestamp && block_timestamp <= self.cf_end_timestamp {
            if &self.cf_progress < &self.cf_target_min {
                return ProjectFundingState::CFActive;
            }
        }

        if block_timestamp > self.cf_end_timestamp {
            if &self.cf_progress < &self.cf_target_min {
                return ProjectFundingState::CFFailed;
            } else if &self.cf_progress - amount_cooling_off >= self.cf_target_min {
                return ProjectFundingState::CFSuccessful;
            } else {
                return ProjectFundingState::CFWaitingCooloff;
            }
        }

        ProjectFundingState::Invalid
    }

    pub fn get_accrued_interest(&self, block_timestamp: u64) -> BigUint<M> {
        if block_timestamp < self.loan_start_timestamp {
            return BigUint::zero();
        }

        let days = (block_timestamp - self.loan_start_timestamp) / (24 * 3600);

        let interest = self
            .cf_progress
            .clone()
            .mul(self.daily_interest_rate)
            .mul(days)
            .div(INTEREST_RATE_DENOMINATION);

        interest
    }

    pub fn get_expected_loan_repayment_timestamp(&self) -> u64 {
        self.loan_start_timestamp + self.loan_duration
    }

    pub fn get_accrued_penalty(&self, block_timestamp: u64) -> BigUint<M> {
        let expected_loan_repayment_timestamp = self.get_expected_loan_repayment_timestamp();
        if block_timestamp < expected_loan_repayment_timestamp {
            return BigUint::zero();
        }

        let days = (block_timestamp - expected_loan_repayment_timestamp) / (24 * 3600);

        let penalty = self
            .cf_progress
            .clone()
            .mul(self.daily_penalty_fee_rate)
            .mul(days)
            .div(INTEREST_RATE_DENOMINATION);

        penalty
    }

    pub fn get_total_amount_due(&self, block_timestamp: u64) -> BigUint<M> {
        let interest = self.get_accrued_interest(block_timestamp);
        let penalty = self.get_accrued_penalty(block_timestamp);

        &self.cf_progress + &interest + &penalty
    }

    pub fn get_repayment_rate(&self, repayment_amount: &BigUint<M>) -> BigUint<M> {
        repayment_amount * INTEREST_RATE_DENOMINATION / &self.cf_progress
    }
}

// Todo: impl getTotalSupply based on cf_progress and share_price_unit

// user -> trimite bani la escrow -> escrow cumpara shares ca proxy
// bot -> call escrow to release shares to user
#[derive(TopEncode, TopDecode, TypeAbi, NestedDecode, NestedEncode, PartialEq, Eq, Clone)]
pub enum ProjectFundingState {
    Invalid = 0,
    Pending = 1,
    CFActive = 2,
    CFWaitingCooloff = 3,
    CFSuccessful = 4,
    CFFailed = 5,
    CFCancelled = 6,
    LoanActive = 7,
    Completed = 8,
}

// claim:
// claim repayment + interest
// claim funds back (if CF failed or cancelled)

// 2 phases:
// - crowdfunding
// - loan repayment
