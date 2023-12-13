// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           30
// Async Callback:                       1
// Total number of exported functions:  32

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
        invest => invest
        getDebugSourceRepaymentSc => get_source_repayment_sc
        withdraw => withdraw
        claim => claim
        distributeRepayment => distribute_repayment
        getProjectDetails => get_project_details
        addAdmins => add_admins
        removeAdmins => remove_admins
        getIsAddressAdmin => is_address_admin
        getAdminList => admin_list
        getLoanShareTokenIdentifier => loan_share_token_identifier
        getCrowdfundingState => crowdfunding_state
        getProjectIdByLoanShareNonce => project_id_by_loan_share_nonce
        getSourceLoanRepaymentScAddress => source_loan_repayment_sc_address
        getRecordedPayments => recorded_payments
        getRepaymentRate => repayment_rates
        getIsKycCompliant => get_is_kyc_compliant
        registerSuccessfulKyc => register_successful_kyc
        getWhitelistedUsers => whitelisted_users
        create => create_project
        cancel => cancel_project
        adminDistributeRepayment => admin_distribute_repayments
        issueAndSetRoles => issue_and_set_roles
        setTransferRole => set_transfer_role
        claimLoanFunds => claim_loan_funds
        getExpectedInterest => get_expected_interest
        getExpectedLateFees => get_expected_late_fees
        getTotalAmount => get_total_amount
        getFundingState => get_funding_state
        getAggregatedCoolOffAmount => get_aggregated_cool_off_amount
    )
}

multiversx_sc_wasm_adapter::async_callback! { loan_crowdfund_sc }
