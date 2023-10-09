multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const INTEREST_RATE_DENOMINATION: u64 = 1_000_000_000;

#[derive(TopEncode, TopDecode, TypeAbi)]
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

    pub is_cancelled: bool, // add loan start time
                            // add expected loan end time
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
        loan_start_timestamp: u64,
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
            loan_start_timestamp,
            repayment_contract_address,
            is_cancelled: false,
        }
    }

    pub fn get_funding_state(&self) -> ProjectFundingState {
        ProjectFundingState::Invalid
    }
}

// Todo: impl getTotalSupply based on cf_progress and share_price_unit

// user -> trimite bani la escrow -> escrow cumpara shares ca proxy
// bot -> call escrow to release shares to user
#[derive(TopEncode, TopDecode, TypeAbi, NestedDecode, NestedEncode)]
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
