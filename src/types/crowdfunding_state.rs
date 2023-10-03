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
    // add loan start time
    // add expected loan end time
}

// Todo: impl getTotalSupply based on cf_progress and share_price_unit

// user -> trimite bani la escrow -> escrow cumpara shares ca proxy
// bot -> call escrow to release shares to user

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
