mod combat;

pub fn imgui(_not_charsel_or_loading: bool) {
    // TODO: Dispatch this somewhere
    // log::info!("imgui: {}", not_charsel_or_loading);
    // This gets spammed a lot
}

pub use combat::wrapped_cbt as combat;
pub use combat::wrapped_cbt_local as combat_local;
