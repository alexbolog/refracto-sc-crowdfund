#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
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

    // see if needed
    // #[payable("*")]
    // #[endpoint(depositRepayment)]
    // fn deposit_repayment(&self) { }

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
