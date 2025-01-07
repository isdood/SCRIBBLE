// freezer.rs

use crate::splat::{self, SplatLevel};
use spin::Mutex;
use lazy_static::lazy_static;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use crate::rtc::DateTime;

// System Constants
const MAX_THAW_ATTEMPTS: u32 = 3;
const FREEZE_TIMEOUT_SECONDS: u64 = 300; // 5 minutes
const SYSTEM_CREATION_DATE: &str = "2025-01-07 06:08:47";
const SYSTEM_CREATOR: &str = "isdood";

#[derive(Debug, Clone)]
pub struct FreezeUser {
    username: String,
    freeze_code_hash: u64,
    is_admin: bool,
    last_thaw: u64,
    created_by: String,
    created_at: String,
}

#[derive(Debug)]
pub enum FreezeError {
    InvalidFreezeCode,
    SystemFrozen,
    TooManyThawAttempts,
    SystemError,
    Unauthorized,
}

struct FreezeState {
    thaw_attempts: AtomicU32,
    last_attempt_time: u64,
    users: Vec<FreezeUser>,
    active_user: Option<String>,
    system_init_time: String,
}

lazy_static! {
    static ref FREEZE_STATE: Mutex<FreezeState> = Mutex::new(FreezeState {
        thaw_attempts: AtomicU32::new(0),
                                                             last_attempt_time: 0,
                                                             users: Vec::new(),
                                                             active_user: None,
                                                             system_init_time: String::from(SYSTEM_CREATION_DATE),
    });
}

fn hash_freeze_code(code: &str) -> u64 {
    let mut hash: u64 = 0;
    for (i, byte) in code.bytes().enumerate() {
        hash = hash.wrapping_add((byte as u64).wrapping_mul(31u64.pow(i as u32)));
    }
    hash
}

pub fn init() {
    let mut state = FREEZE_STATE.lock();

    // Add default user
    state.users.push(FreezeUser {
        username: String::from("slug"),
                     freeze_code_hash: hash_freeze_code("123"),
                     is_admin: true,
                     last_thaw: 0,
                     created_by: String::from(SYSTEM_CREATOR),
                     created_at: String::from(SYSTEM_CREATION_DATE),
    });

    splat::log(
        SplatLevel::BitsNBytes,
        &format!(
            "CryoSys Initialized:\n\
└─ Creation: {}\n\
└─ Creator: {}\n\
└─ Status: Frozen\n\
└─ Default User: slug",
SYSTEM_CREATION_DATE,
SYSTEM_CREATOR
        )
    );
}

pub fn thaw(username: &str, freeze_code: &str) -> Result<(), FreezeError> {
    let mut state = FREEZE_STATE.lock();
    let current_time = crate::rtc::DateTime::now().to_string();

    // Check for timeout due to too many attempts
    if state.thaw_attempts.load(Ordering::Relaxed) >= MAX_THAW_ATTEMPTS {
        let time_since_last = current_time.parse::<u64>()
        .unwrap_or(0)
        .saturating_sub(state.last_attempt_time);

        if time_since_last < FREEZE_TIMEOUT_SECONDS {
            splat::log(
                SplatLevel::Warning,
                &format!(
                    "System frozen for {} more seconds",
                    FREEZE_TIMEOUT_SECONDS - time_since_last
                )
            );
            return Err(FreezeError::SystemFrozen);
        } else {
            state.thaw_attempts.store(0, Ordering::Relaxed);
        }
    }

    // Verify credentials
    let user = state.users.iter_mut().find(|u| u.username == username);
    match user {
        Some(user) if user.freeze_code_hash == hash_freeze_code(freeze_code) => {
            user.last_thaw = current_time.parse().unwrap_or(0);
            state.active_user = Some(username.to_string());
            state.thaw_attempts.store(0, Ordering::Relaxed);

            splat::log(
                SplatLevel::BitsNBytes,
                &format!(
                    "System thawed:\n\
└─ User: {}\n\
└─ Time: {}\n\
└─ Admin: {}",
username,
current_time,
user.is_admin
                )
            );
            Ok(())
        }
        _ => {
            let attempts = state.thaw_attempts.fetch_add(1, Ordering::Relaxed) + 1;
            state.last_attempt_time = current_time.parse().unwrap_or(0);

            splat::log(
                SplatLevel::Warning,
                &format!(
                    "Thaw attempt failed:\n\
└─ User: {}\n\
└─ Attempts: {}/{}\n\
└─ Time: {}",
username,
attempts,
MAX_THAW_ATTEMPTS,
current_time
                )
            );

            Err(FreezeError::InvalidFreezeCode)
        }
    }
}

pub fn freeze() {
    let mut state = FREEZE_STATE.lock();
    if let Some(username) = state.active_user.take() {
        splat::log(
            SplatLevel::BitsNBytes,
            &format!(
                "System frozen:\n\
└─ Last User: {}\n\
└─ Time: {}",
username,
crate::rtc::DateTime::now().to_string()
            )
        );
    }
}

pub fn change_freeze_code(username: &str, old_code: &str, new_code: &str) -> Result<(), FreezeError> {
    let mut state = FREEZE_STATE.lock();

    if let Some(user) = state.users.iter_mut().find(|u| u.username == username) {
        if user.freeze_code_hash == hash_freeze_code(old_code) {
            user.freeze_code_hash = hash_freeze_code(new_code);
            splat::log(
                SplatLevel::BitsNBytes,
                &format!(
                    "Freeze code updated:\n\
└─ User: {}\n\
└─ Time: {}\n\
└─ Updated By: {}",
username,
crate::rtc::DateTime::now().to_string(),
                         state.active_user.as_ref().unwrap_or(&String::from("SYSTEM"))
                )
            );
            Ok(())
        } else {
            Err(FreezeError::InvalidFreezeCode)
        }
    } else {
        Err(FreezeError::InvalidFreezeCode)
    }
}

pub fn get_active_user() -> Option<String> {
    FREEZE_STATE.lock().active_user.clone()
}

pub fn is_frozen() -> bool {
    FREEZE_STATE.lock().active_user.is_none()
}

pub fn get_cryo_status() -> String {
    let state = FREEZE_STATE.lock();
    format!(
        "CryoSystem Status:\n\
└─ Creation Date: {}\n\
└─ Created By: {}\n\
└─ Status: {}\n\
└─ Active User: {}\n\
└─ Failed Thaws: {}\n\
└─ Users: {}",
state.system_init_time,
SYSTEM_CREATOR,
if is_frozen() { "Frozen" } else { "Thawed" },
    state.active_user.as_ref().unwrap_or(&String::from("None")),
            state.thaw_attempts.load(Ordering::Relaxed),
            state.users.len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thaw_freeze() {
        init();
        assert!(thaw("slug", "123").is_ok());
        assert!(!is_frozen());
        freeze();
        assert!(is_frozen());
    }

    #[test]
    fn test_failed_thaw() {
        init();
        assert!(thaw("slug", "wrong").is_err());
        assert!(is_frozen());
    }
}
