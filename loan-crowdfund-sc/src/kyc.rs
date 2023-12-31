use crate::constants::ERR_KYC_NOT_DONE;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait KycModule: crate::permissions::PermissionsModule {
    #[view(getIsKycCompliant)]
    fn get_is_kyc_compliant(&self, address: &ManagedAddress) -> bool {
        self.whitelisted_users().contains(address)
    }

    fn require_address_is_kyc_compliant(&self, address: &ManagedAddress) {
        require!(self.get_is_kyc_compliant(address), ERR_KYC_NOT_DONE);
    }

    #[endpoint(registerSuccessfulKyc)]
    fn register_successful_kyc(&self, address: ManagedAddress) {
        // TODO: uncomment this line after demo
        // self.require_address_is_admin(&self.blockchain().get_caller());
        self.whitelisted_users().insert(address);
    }

    #[view(getWhitelistedUsers)]
    #[storage_mapper("whitelisted_users")]
    fn whitelisted_users(&self) -> SetMapper<ManagedAddress>;
}
