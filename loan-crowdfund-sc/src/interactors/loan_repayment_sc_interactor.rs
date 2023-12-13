multiversx_sc::imports!();
use loan_refund_escrow_sc::ProxyTrait as _;

#[multiversx_sc::module]
pub trait LoanRepaymentScInteractor {
    fn withdraw_repayment_funds(&self, sc_address: ManagedAddress) -> BigUint {
        self.repayment_sc_proxy(sc_address)
            .withdraw_repayment_funds()
            .execute_on_dest_context()
    }

    fn get_repayment_funds_balance(&self, sc_address: ManagedAddress) -> BigUint {
        self.repayment_sc_proxy(sc_address)
            .get_repayment_funds_balance()
            .execute_on_dest_context()
    }

    #[proxy]
    fn repayment_sc_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> loan_refund_escrow_sc::Proxy<Self::Api>;
}
