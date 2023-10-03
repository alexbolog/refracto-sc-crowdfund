multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::permissions::PermissionsModule {
    #[endpoint(create)]
    fn create_project(&self) {}

    #[endpoint(cancel)]
    fn cancel_project(&self) {}

    #[endpoint(adminDistributeRepayment)]
    fn admin_distribute_repayments(&self) {}
}
