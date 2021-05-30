use crate::{exports::*, main, release};
use winapi::shared::{minwindef::LPVOID, ntdef::PCCHAR};
use std::sync::mpsc::{Sender};

pub fn gen_arcdps(tx: Sender<String>) -> LPVOID {
    let i = imgui as arcdps_bindings::SafeImguiCallback;
    let c = combat(tx.clone());// as arcdps_bindings::SafeCombatCallback;
    let l = combat_local(tx.clone());//as arcdps_bindings::SafeCombatCallback;
    arcdps_bindings::arcdps_exports::new(0x2_0804 | 0x4650 << 32, "BHUDrender", env!("CARGO_PKG_VERSION"))
        .imgui(i)
        .combat(c as arcdps_bindings::SafeCombatCallback)
        .combat_local(l)
        .save() as LPVOID
}

#[no_mangle]
/* export -- arcdps looks for this exported function and calls the address it returns on client load */
pub extern "system" fn get_init_addr(
    _arcversionstr: PCCHAR,
    _imguicontext: LPVOID,
    _id3dd9: LPVOID,
) -> LPVOID {
    main as LPVOID
}

#[no_mangle]
/* export -- arcdps looks for this exported function and calls the address it returns on client exit */
pub extern "system" fn get_release_addr() -> LPVOID {
    release as LPVOID
}