use crate::modules::parser::data_structs::log::{BeginTrial, EndTrial, TrialID, Trialinit, ZoneInfo};

use super::combat::EncounterGroups;


#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Trial {
    pub id: TrialID,
    pub in_progress: bool,
    pub completed: bool,
    pub start_time_ms: Option<String>, //BigInt Stored as String due to IPC limitations
    pub duration_ms: Option<u32>,
    pub success: bool,
    pub final_score: Option<u32>,
    pub vitality: Option<u8>,
}

impl Trial {
    pub fn from_init(value: Trialinit) -> Option<Self> {
        if !value.in_progress && !value.completed {
            None
        } else {
            Some(Self {
                id: value.id,
                in_progress: value.in_progress,
                completed: value.completed,
                start_time_ms: if value.start_time_ms == *"0" {
                    None
                } else {
                    Some(value.start_time_ms)
                }, //BigInt Stored as String due to IPC limitations
                duration_ms: if value.duration_ms == 0 {
                    None
                } else {
                    Some(value.duration_ms)
                },
                success: value.success,
                final_score: if value.final_score == 0 {
                    None
                } else {
                    Some(value.final_score)
                },
                vitality: None,
            })
        }
    }
    pub fn from_begin(value: BeginTrial) -> Self {
        Self {
            id: value.id,
            in_progress: true,
            completed: false,
            start_time_ms: Some(value.start_time_ms), //BigInt Stored as String due to IPC limitations
            duration_ms: None,
            success: false,
            final_score: None,
            vitality: None,
        }
    }

    pub fn merge_end(&self, value: EndTrial) -> Self {
        let mut out = self.clone();
        out.in_progress = false;
        out.completed = true;
        out.duration_ms = Some(value.duration_ms);
        out.success = value.success;
        out.final_score = Some(value.final_score);
        out.vitality = Some((value.final_vitality_bonus / 1000) as u8);
        out
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Zone {
    pub zone: ZoneInfo,
    pub trial: Option<Trial>,
    pub encounters: Vec<EncounterGroups>,
}

