// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use stylus_toolkit::tokens::erc20::{Erc20, Erc20Params};
use alloy_primitives::U256;
use stylus_sdk::{
    prelude::*,
    stylus_proc::entrypoint,
    msg,
};

struct MyParams;
impl Erc20Params for MyParams {
    const NAME: &'static str = "Dummy ERC20 token";
    const SYMBOL: &'static str = "DERC20";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint] // Makes Weth the entrypoint
    struct DummyErc20 {
        #[borrow] // Allows erc20 to access Dummy Erc20's storage and make calls
        Erc20<MyParams> erc20;
    }
}
#[external]
#[inherit(Erc20<MyParams>)]
impl DummyErc20 {

    pub fn fund(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20._mint(msg::sender(), amount);
        Ok(())
    }

}