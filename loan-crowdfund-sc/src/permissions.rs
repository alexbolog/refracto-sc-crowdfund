multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PermissionsModule {
    #[endpoint(addAdmins)]
    fn add_admins(&self, addresses: MultiValueManagedVec<ManagedAddress>) {
        self.require_caller_is_admin();

        for address in addresses.iter() {
            self.admin_list().insert(address.clone_value());
        }
    }

    #[endpoint(removeAdmins)]
    fn remove_admins(&self, addresses: MultiValueManagedVec<ManagedAddress>) {
        self.require_caller_is_admin();

        for address in addresses.iter() {
            self.admin_list().remove(&address);
        }
    }

    #[view(getIsAddressAdmin)]
    fn is_address_admin(&self, address: &ManagedAddress) -> bool {
        self.admin_list().contains(address) || address == &self.blockchain().get_caller()
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
