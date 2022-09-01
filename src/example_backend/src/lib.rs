use ic_cdk_macros::{update};
use ic_cdk::api::{self, caller, call::call};
use ic_cdk::api::management_canister::main as mc_m;
use ic_cdk::export::candid::{candid_method, CandidType, Deserialize, Principal};
use ic_kit::{ic};
use ic_kit::interfaces::management::InstallMode;
use ic_kit::interfaces::{management};

use serde::Serialize;
use serde_bytes::ByteBuf;

const CYCLE_SHARE: u128 = 1_000_000_000_000; // 1T Cycles

pub const WASM: &[u8] =
    include_bytes!("../../../wasm/hello_backend.wasm");

#[derive(CandidType, Deserialize)]
pub struct InstallCodeArgumentBorrowed<'a> {
    pub mode: InstallMode,
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
    pub wasm_module: &'a [u8],
    pub arg: Vec<u8>,
}

#[update(name = "createAndInstall")]
#[candid_method(update, rename = "createAndInstall")]
pub async fn create_and_install() -> String {

    use management::{CanisterStatus, WithCanisterId};

    let canister_settings = mc_m::CanisterSettings {
        controllers: Some(vec![api::id(), caller()]),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None
    };

    let create_arg =  mc_m::CreateCanisterArgument{
        settings : Some(canister_settings),
    };

    let cid = mc_m::create_canister(create_arg).await.unwrap().0;
    let canister_id = cid.canister_id;

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Install,
        canister_id,
        wasm_module: WASM,
        arg: vec![],
    };

    let _: () = ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    ).await.expect("Install code failed.");

    let _ = mc_m::deposit_cycles(cid, CYCLE_SHARE).await.unwrap();

    Principal::to_text(&canister_id).to_string()
}