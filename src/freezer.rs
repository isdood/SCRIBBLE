// src/freezer.rs
use alloc::string::{String, ToString};
use core::sync::atomic::Ordering;
use spin::Mutex;

// Define system constants
const SYSTEM_CREATOR: &str = "isdood";
const SYSTEM_CREATION_DATE: &str = "2025-01-07 07:32:34";

#[derive(Debug)]
pub struct User {
    username: String,
    is_admin: bool,
    created_by: String,
    created_at: String,
}

#[derive(Debug)]
pub struct FreezerState {
    users: Vec<User>,
    active_user: Option<String>,
    system_init_time: String,
    thaw_attempts: AtomicUsize,
}

impl FreezerState {
    pub fn new() -> Self {
        let mut state = FreezerState {
            users: Vec::new(),
            active_user: None,
            system_init_time: String::from(SYSTEM_CREATION_DATE),
            thaw_attempts: AtomicUsize::new(0),
        };

        // Add default admin user
        state.users.push(User {
            username: String::from("slug"),
                         is_admin: true,
                         created_by: String::from(SYSTEM_CREATOR),
                         created_at: String::from(SYSTEM_CREATION_DATE),
        });

        state
    }
}

lazy_static::lazy_static! {
    static ref STATE: Mutex<FreezerState> = Mutex::new(FreezerState::new());
}

pub fn login(username: &str) -> bool {
    let mut state = STATE.lock();

    // First find the user and check if they exist
    let user_exists = state.users.iter().any(|u| u.username == username);

    if !user_exists {
        return false;
    }

    // If we get here, the user exists, so update the state
    state.active_user = Some(username.to_string());
    state.thaw_attempts.store(0, Ordering::Relaxed);

    // Get admin status after releasing mutable borrow
    let is_admin = state.users.iter()
    .find(|u| u.username == username)
    .map(|u| u.is_admin)
    .unwrap_or(false);

    is_admin
}

pub fn is_admin(username: &str) -> bool {
    let state = STATE.lock();
    state.users.iter()
    .find(|u| u.username == username)
    .map(|u| u.is_admin)
    .unwrap_or(false)
}

pub fn get_active_user() -> Option<String> {
    STATE.lock().active_user.clone()
}

pub fn get_cryo_status() -> String {
    let state = STATE.lock();
    let active_user = state.active_user.as_ref()
    .map(String::as_str)
    .unwrap_or("None");

    format!(
        "System Status:\n\
Active User: {}\n\
System Init: {}\n\
Thaw Attempts: {}",
active_user,
state.system_init_time,
state.thaw_attempts.load(Ordering::Relaxed)
    )
}
