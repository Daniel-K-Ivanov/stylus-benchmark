// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use alloc::vec::Vec;
use alloy_primitives::FixedBytes;
use stylus_sdk::{
    abi::Bytes,
    alloy_sol_types::{sol, SolError},
    evm,
    prelude::*,
    stylus_proc::entrypoint,
};
use stylus_toolkit::crypto::ed25519::ed25519_verify;

sol_storage! {
    #[entrypoint]
    struct Ed25519VerifyBenchmark { }
}

sol! {
    event VerificationResult(bool success);
}

#[external]
impl Ed25519VerifyBenchmark {
    pub fn verify(
        &mut self,
        msg: Bytes,
        signature: Bytes,
        public_key: FixedBytes<32>,
    ) -> Result<(), Vec<u8>> {
        let result = ed25519_verify(public_key, signature, msg);
        evm::log(VerificationResult { success: result });
        Ok(())
    }
}
