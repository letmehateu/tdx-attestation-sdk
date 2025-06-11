pub mod enclave_id;
pub mod fmspc_tcb;
pub mod pcs;

// Chain Defaults
pub const DEFAULT_RPC_URL: &str = "https://1rpc.io/ata/testnet";
pub const DEFAULT_DCAP_CONTRACT: &str = "95175096a9B74165BE0ac84260cc14Fc1c0EF5FF";

// PCCS addresses
pub const ENCLAVE_ID_DAO_ADDRESS: &str = "d74e880029cd3B6b434f16beA5F53A06989458Ee";
pub const FMSPC_TCB_DAO_ADDRESS: &str = "d3A3f34E8615065704cCb5c304C0cEd41bB81483";
pub const PCS_DAO_ADDRESS: &str = "B270cD8550DA117E3accec36A90c4b0b48daD342";
pub const PCK_DAO_ADDRESS: &str = "a4615C2a260413878241ff7605AD9577feB356A5";

pub fn remove_prefix_if_found(h: &str) -> &str {
    if h.starts_with("0x") {
        &h[2..]
    } else {
        &h
    }
}
