multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CommonModule {
    #[view(getExpectedInterest)]
    fn get_expected_interest(&self) {}

    #[view(getExpectedLateFees)]
    fn get_expected_late_fees(&self) {}

    #[view(getFinalAmount)]
    fn get_final_amount(&self) {}
}
