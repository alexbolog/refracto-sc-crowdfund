multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PermissionsModule {
    fn is_address_admin(&self, address: &ManagedAddress) -> bool {
        self.admin_list().contains(address)
    }

    fn require_caller_is_admin(&self) {
        self.require_address_is_admin(&self.blockchain().get_caller());
    }

    fn require_address_is_admin(&self, address: &ManagedAddress) {
        require!(
            self.is_address_admin(address),
            "Endpoint can only be called by admins"
        );
    }

    #[view(getAdminList)]
    #[storage_mapper("admin_list")]
    fn admin_list(&self) -> SetMapper<ManagedAddress>;
}
