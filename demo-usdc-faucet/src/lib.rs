#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod nft_module;

const ONE_TOKEN_AMOUNT: u64 = 1_000_000_000_000_000_000;
const MINT_AMOUNT: u64 = 20_000;

#[multiversx_sc::contract]
pub trait DemoUsdcFaucet: nft_module::NftModule {
    #[init]
    fn init(&self) {}

    #[endpoint(mint)]
    fn mint(&self) {
        let caller = self.blockchain().get_caller();
        let mint_amount = BigUint::from(MINT_AMOUNT) * BigUint::from(ONE_TOKEN_AMOUNT);
        let token_id = self.token_id().get();

        self.send().esdt_local_mint(&token_id, 0, &mint_amount);

        self.send().direct_esdt(&caller, &token_id, 0, &mint_amount);
    }
}
