use arcdps::*;

unsafe extern "C" fn combat_callback(
    ev: Option<&CombatEvent>,
    _src: Option<&RawAgent>,
    _dst: Option<&RawAgent>,
    _skill_name: *mut i8,
    _id: u64,
    _revision: u64,
) {
    if let Some(event) = ev {
        println!("Combat event: {:?}", event.time);
    } else {
        println!("Plugin initialized");
    }
}

arcdps::arcdps_export! {
    name: "Barebones Plugin",
    sig: 0x12345678,
    raw_combat: combat_callback,
}
