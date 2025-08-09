
use arcdps::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// JSON schema for CombatEvent
pub const COMBAT_EVENT_SCHEMA: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "CombatEvent",
    "type": "object",
    "properties": {
        "time": {
            "description": "Timestamp of the event in milliseconds",
            "type": "integer",
            "minimum": 0
        },
        "src_agent": {
            "description": "Source agent ID",
            "type": "integer",
            "minimum": 0
        },
        "dst_agent": {
            "description": "Destination agent ID",
            "type": "integer",
            "minimum": 0
        },
        "value": {
            "description": "Main value of the event (damage, healing, etc.)",
            "type": "integer"
        },
        "buff_dmg": {
            "description": "Buff damage value",
            "type": "integer"
        },
        "overstack_value": {
            "description": "Overstack value",
            "type": "integer",
            "minimum": 0
        },
        "skill_id": {
            "description": "Skill ID",
            "type": "integer",
            "minimum": 0
        },
        "src_instance_id": {
            "description": "Source instance ID",
            "type": "integer",
            "minimum": 0
        },
        "dst_instance_id": {
            "description": "Destination instance ID",
            "type": "integer",
            "minimum": 0
        },
        "src_master_instance_id": {
            "description": "Source master instance ID",
            "type": "integer",
            "minimum": 0
        },
        "dst_master_instance_id": {
            "description": "Destination master instance ID",
            "type": "integer",
            "minimum": 0
        },
        "iff": {
            "description": "IFF (Identification Friend or Foe) flag",
            "type": "integer",
            "minimum": 0,
            "maximum": 255
        },
        "buff": {
            "description": "Buff ID",
            "type": "integer",
            "minimum": 0,
            "maximum": 255
        },
        "result": {
            "description": "Result flags",
            "type": "integer",
            "minimum": 0,
            "maximum": 255
        },
        "is_activation": {
            "description": "Is activation event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_buff_remove": {
            "description": "Is buff remove event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_ninety": {
            "description": "Is 90% event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_fifty": {
            "description": "Is 50% event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_moving": {
            "description": "Is moving event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_statechange": {
            "description": "Is state change event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_flanking": {
            "description": "Is flanking event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_shields": {
            "description": "Is shields event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        },
        "is_off_cycle": {
            "description": "Is off cycle event",
            "type": "integer",
            "minimum": 0,
            "maximum": 1
        }
    },
    "required": [
        "time", "src_agent", "dst_agent", "value", "buff_dmg", "overstack_value",
        "skill_id", "src_instance_id", "dst_instance_id", "src_master_instance_id",
        "dst_master_instance_id", "iff", "buff", "result", "is_activation",
        "is_buff_remove", "is_ninety", "is_fifty", "is_moving", "is_statechange",
        "is_flanking", "is_shields", "is_off_cycle"
    ]
}"#;

/// Serializable version of CombatEvent for JSON
#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableCombatEvent {
    pub time: u64,
    pub src_agent: usize,
    pub dst_agent: usize,
    pub value: i32,
    pub buff_dmg: i32,
    pub overstack_value: u32,
    pub skill_id: u32,
    pub src_instance_id: u16,
    pub dst_instance_id: u16,
    pub src_master_instance_id: u16,
    pub dst_master_instance_id: u16,
    pub iff: u8,
    pub buff: u8,
    pub result: u8,
    pub is_activation: u8,
    pub is_buff_remove: u8,
    pub is_ninety: u8,
    pub is_fifty: u8,
    pub is_moving: u8,
    pub is_statechange: u8,
    pub is_flanking: u8,
    pub is_shields: u8,
    pub is_off_cycle: u8,
}

impl From<&CombatEvent> for SerializableCombatEvent {
    fn from(event: &CombatEvent) -> Self {
        SerializableCombatEvent {
            time: event.time,
            src_agent: event.src_agent,
            dst_agent: event.dst_agent,
            value: event.value,
            buff_dmg: event.buff_dmg,
            overstack_value: event.overstack_value,
            skill_id: event.skill_id,
            src_instance_id: event.src_instance_id,
            dst_instance_id: event.dst_instance_id,
            src_master_instance_id: event.src_master_instance_id,
            dst_master_instance_id: event.dst_master_instance_id,

            iff: event.iff,
            buff: event.buff,
            result: event.result,
            is_activation: event.is_activation,
            is_buff_remove: event.is_buff_remove,
            is_ninety: event.is_ninety,
            is_fifty: event.is_fifty,
            is_moving: event.is_moving,
            is_statechange: event.is_statechange,
            is_flanking: event.is_flanking,
            is_shields: event.is_shields,
            is_off_cycle: event.is_off_cycle,
        }
    }
}

/// Convert CombatEvent to JSON
pub fn combat_event_to_json(event: &CombatEvent) -> Value {
    let serializable: SerializableCombatEvent = event.into();
    serde_json::to_value(serializable).unwrap()
}
