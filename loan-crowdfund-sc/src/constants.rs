pub const COOL_OFF_PERIOD: u64 = 3600 * 24 * 14; // 14 days

pub const ERR_COOL_OFF_EXPIRED: &str = "Cannot withdraw, cool off period expired";
pub const ERR_CANNOT_INVEST_IN_CRT_STATE: &str = "Cannot invest in current funding state";
pub const ERR_CANNOT_OVER_FINANCE: &str = "Investment exceeds project goal";
pub const ERR_KYC_NOT_DONE: &str = "KYC not verified";
pub const ERR_INVALID_PROJECT_ID: &str = "Invalid project id";
pub const ERR_INVALID_PAYMENT_TOKEN: &str = "Invalid payment token";
pub const ERR_INVALID_PAYMENT_NONCE: &str = "Invalid payment nonce";
pub const ERR_INVESTMENT_NOT_FOUND: &str = "Investment not found";
pub const ERR_CANNOT_WITHDRAW_IN_CRT_STATE: &str = "Cannot withdraw in current funding state";
pub const ERR_REPAYMENT_DISTRIBUTED: &str = "Repayment already distributed";
pub const ERR_TOKEN_ISSUED: &str = "Token already issued";
pub const ERR_INSUFFICIENT_REPAYMENT_AMOUNT: &str = "Insufficient repayment amount";
pub const ERR_CANNOT_CLAIM_IN_CRT_STATE: &str = "Cannot claim in current funding state";
pub const ERR_NOTHING_TO_CLAIM: &str = "Nothing to claim";
