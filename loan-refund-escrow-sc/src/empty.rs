#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait LoanRefundEscrowScContract {
    #[init]
    fn init(
        &self,
        project_id: u64,
        developer_wallet_address: ManagedAddress,
        loan_repayment_token_id: TokenIdentifier,
    ) {
        self.project_id().set_if_empty(project_id);
        self.developer_wallet_address()
            .set_if_empty(developer_wallet_address);
        self.loan_repayment_token_id()
            .set_if_empty(loan_repayment_token_id);
    }

    #[payable("*")]
    #[endpoint(depositLoanRepayment)]
    fn deposit_loan_repayment(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        self.require_address_is_developer_or_admin(&caller);
        self.require_payment_token_is_loan_repayment_token(&payment.token_identifier);
    }

    #[endpoint(withdrawRepaymentFunds)]
    fn withdraw_repayment_funds(&self) -> BigUint {
        let caller = self.blockchain().get_caller();
        self.require_address_is_developer_or_admin(&caller);

        let token_id = self.loan_repayment_token_id().get();
        let balance = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(token_id.clone()), 0);
        self.send().direct_esdt(&caller, &token_id, 0, &balance);

        balance
    }

    fn require_payment_token_is_loan_repayment_token(&self, token_identifier: &TokenIdentifier) {
        require!(
            token_identifier == &self.loan_repayment_token_id().get(),
            "Invalid token identifier used for deposit"
        );
    }

    fn require_address_is_developer_or_admin(&self, address: &ManagedAddress) {
        let owner_address = self.blockchain().get_owner_address();
        let is_admin: bool = self
            .cf_contract_proxy(owner_address.clone())
            .is_address_admin(address.clone())
            .execute_on_dest_context();
        require!(
            is_admin
                || address == &owner_address
                || address == &self.developer_wallet_address().get(),
            "Only project developer/admins can call this function"
        );
    }

    #[view(getRepaymentFundsBalance)]
    fn get_repayment_funds_balance(&self) -> BigUint {
        let token_id = self.loan_repayment_token_id().get();
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(token_id), 0)
    }

    #[proxy]
    fn cf_contract_proxy(&self, sc_address: ManagedAddress)
        -> crowdfund_sc_proxy::Proxy<Self::Api>;

    #[view(getProjectId)]
    #[storage_mapper("project_id")]
    fn project_id(&self) -> SingleValueMapper<u64>;

    #[view(getDeveloperWalletAddress)]
    #[storage_mapper("developer_wallet_address")]
    fn developer_wallet_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getLoanRepaymentTokenId)]
    #[storage_mapper("loan_repayment_token_id")]
    fn loan_repayment_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}

/*
#[view(getIsAddressAdmin)]
    fn is_address_admin(&self, address: &ManagedAddress) -> bool
 */

mod crowdfund_sc_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait CrowdfundSc {
        #[view(getIsAddressAdmin)]
        fn is_address_admin(&self, address: &ManagedAddress) -> bool;
    }
}
