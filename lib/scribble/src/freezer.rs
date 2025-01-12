// src/freezer.rs
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

const MAX_THAW_ATTEMPTS: u32 = 3;
const SYSTEM_CREATION_DATE: &str = "2025-01-07 07:49:01";

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub is_admin: bool,
    created_by: String,
    created_at: String,
}

#[derive(Debug)]
pub struct FreezerState {
    pub created_by: String,
    pub system_init_time: String,
    pub active_user: Option<String>,
    pub thaw_attempts: AtomicU32,
    pub users: Vec<User>,
}

lazy_static! {
    static ref FREEZER_STATE: Mutex<FreezerState> = Mutex::new(FreezerState::new());
}

impl FreezerState {
    pub fn new() -> Self {
        FreezerState {
            created_by: String::from("system"),
            system_init_time: String::from(SYSTEM_CREATION_DATE),
            active_user: None,
            thaw_attempts: AtomicU32::new(0),
            users: Vec::new(),
        }
    }
}

pub fn login(username: &str) -> bool {
    let state = FREEZER_STATE.lock();

    if state.thaw_attempts.load(Ordering::Relaxed) >= MAX_THAW_ATTEMPTS {
        return false;
    }

    let user_exists = state.users.iter().any(|u| u.username == username);

    if !user_exists {
        state.thaw_attempts.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    // Reset thaw attempts on successful login
    state.thaw_attempts.store(0, Ordering::Relaxed);

    // Check if user is admin
    state.users.iter()
    .find(|u| u.username == username)
    .map(|u| u.is_admin)
    .unwrap_or(false)
}
