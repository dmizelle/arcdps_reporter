


use arcdps::*;
use serde_json::json;
use std::ptr;

mod combat_event_schema;
use combat_event_schema::{combat_event_to_json, COMBAT_EVENT_SCHEMA};

mod http_client;
use http_client::send_combat_event;

fn main() {
    println!("Testing combat event JSON serialization...");

    // Create a mock combat event
    let mock_event = CombatEvent {
        time: 123456789,
        src_agent: 1001,
        dst_agent: 2002,
        value: 5000,
        buff_dmg: 1000,
        overstack_value: 0,
        skill_id: 12345,
        src_instance_id: 1,
        dst_instance_id: 2,
        src_master_instance_id: 10,
        dst_master_instance_id: 20,
        iff: 1,
        buff: 0,
        result: 0,
        is_activation: 0,
        is_buff_remove: 0,
        is_ninety: 0,
        is_fifty: 0,
        is_moving: 0,
        is_statechange: 0,
        is_flanking: 0,
        is_shields: 0,
        is_off_cycle: 0,
        pad61: 0,
        pad62: 0,
        pad63: 0,
        pad64: 0,
    };

    // Create mock agents
    let mock_src_agent = RawAgent {
        name: ptr::null_mut(),
        id: 1001,
        prof: 1,
        elite: 0,
        self_: 1,
        team: 0,
    };

    let mock_dst_agent = RawAgent {
        name: ptr::null_mut(),
        id: 2002,
        prof: 5,
        elite: 0,
        self_: 0,
        team: 1,
    };

    // Test the callback function
    unsafe {
        combat_callback(
            Some(&mock_event),
            Some(&mock_src_agent),
            Some(&mock_dst_agent),
            b"Fireball\0".as_ptr() as *mut i8,
            0,
            0,
        );
    }

    // Test initialization
    unsafe {
        combat_callback(None, None, None, ptr::null_mut(), 0, 0);
    }
}

// Copy of the actual callback function for testing
unsafe fn combat_callback(
    ev: Option<&CombatEvent>,
    src: Option<&RawAgent>,
    dst: Option<&RawAgent>,
    skill_name: *mut i8,
    _id: u64,
    _revision: u64,
) {
    if let Some(event) = ev {
        // Convert the combat event to JSON
        let event_json = combat_event_to_json(event);

        // Build the full event JSON manually to avoid serialization issues
        let mut full_event = json!({
            "event": event_json,
            "src_agent": null,
            "dst_agent": null,
            "skill_name": null,
        });

        // Add source agent info if available
        if let Some(src_agent) = src {
            full_event["src_agent"] = json!({
                "id": src_agent.id,
                "prof": src_agent.prof,
                "elite": src_agent.elite,
                "self_": src_agent.self_,
                "team": src_agent.team,
            });
        }

        // Add destination agent info if available
        if let Some(dst_agent) = dst {
            full_event["dst_agent"] = json!({
                "id": dst_agent.id,
                "prof": dst_agent.prof,
                "elite": dst_agent.elite,
                "self_": dst_agent.self_,
                "team": dst_agent.team,
            });
        }

        // Add skill name if available
        if !skill_name.is_null() {
            // Safety: skill_name is a null-terminated C string
            let c_str = std::ffi::CStr::from_ptr(skill_name);
            if let Ok(skill_name_str) = c_str.to_str() {
                full_event["skill_name"] = json!(skill_name_str);
            }
        }

        // Print the JSON
        println!("Combat event JSON: {}", serde_json::to_string_pretty(&full_event).unwrap());

        // Send to HTTP endpoint
        let endpoint = "http://localhost:51815/combat-events";
        match send_combat_event(endpoint, &full_event) {
            Ok(_) => println!("Successfully sent combat event to {}", endpoint),
            Err(e) => eprintln!("Failed to send combat event: {}", e),
        }
    } else {
        println!("Plugin initialized");
        // Print the schema when plugin is initialized
        println!("Combat Event Schema: {}", COMBAT_EVENT_SCHEMA);
    }
}


