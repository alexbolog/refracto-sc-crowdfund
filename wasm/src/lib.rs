// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    loan_crowdfund_sc
    (
        init => init
        getAdminList => admin_list
        getWithdrawFromLoanTimeSpan => withdraw_from_loan_timespan
        getLoanShareTokenIdentifiers => loan_share_token_identifiers
        getCrowdfundingState => crowdfunding_state
        getIsKycCompliant => get_is_kyc_compliant
        registerSuccessfulKyc => register_successful_kyc
        getWhitelistedUsers => whitelisted_users
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
