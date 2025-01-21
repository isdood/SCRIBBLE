// src/engine.rs
// Created: 2025-01-21 18:45:47 UTC
// Author: isdood

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    time::{Duration, Instant},
};

use parking_lot::{RwLock, Mutex};
use rayon::ThreadPoolBuilder;
use thiserror::Error;
use log::{info, warn, error};

use crate::{
    core::{
        crystal::{Crystal, CrystalConfig, CrystalError},
        systems::SystemManager,
        parallel::TaskScheduler,
    },
    render::{
        RenderManager,
        vulkan::VulkanContext,
    },
    physics::{
        PhysicsWorld,
        harmony::HarmonySimulator,
    },
};

/// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub crystal: CrystalConfig,
    pub thread_count: usize,
    pub vsync: bool,
    pub physics_timestep: Duration,
    pub max_frame_time: Duration,
    pub debug_logging: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            crystal: CrystalConfig::default(),
            thread_count: num_cpus::get(),
            vsync: true,
            physics_timestep: Duration::from_secs_f64(1.0 / 60.0),
            max_frame_time: Duration::from_secs_f64(1.0 / 30.0),
            debug_logging: false,
        }
    }
}

/// Engine errors
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    #[error("Crystal error: {0}")]
    Crystal(#[from] CrystalError),
    #[error("Renderer error: {0}")]
    Render(String),
    #[error("Physics error: {0}")]
    Physics(String),
    #[error("System error: {0}")]
    System(String),
}

/// Engine result type
pub type EngineResult<T> = Result<T, EngineError>;

/// Main engine state
#[derive(Debug)]
pub struct Engine {
    config: EngineConfig,
    crystal: Arc<RwLock<Crystal>>,
    renderer: Arc<RenderManager>,
    physics: Arc<PhysicsWorld>,
    systems: Arc<SystemManager>,
    scheduler: Arc<TaskScheduler>,
    harmony_sim: Arc<HarmonySimulator>,
    running: Arc<AtomicBool>,
    frame_count: Arc<AtomicU64>,
    last_frame: Arc<Mutex<Instant>>,
    frame_times: Arc<RwLock<Vec<Duration>>>,
}

impl Engine {
    /// Create a new engine instance
    pub fn new(config: EngineConfig) -> EngineResult<Self> {
        // Initialize logging
        if config.debug_logging {
            env_logger::init();
        }

        info!("Initializing Prismancer Engine v0.1.0");
        info!("Build: 2025-01-21 18:45:47 UTC by isdood");

        // Setup thread pool
        ThreadPoolBuilder::new()
        .num_threads(config.thread_count)
        .thread_name(|i| format!("prismancer-{}", i))
        .build_global()
        .map_err(|e| EngineError::InitializationFailed(e.to_string()))?;

        // Initialize core systems
        let crystal = Arc::new(RwLock::new(Crystal::new(config.crystal.clone())));
        let vulkan = VulkanContext::new()
        .map_err(|e| EngineError::Render(e.to_string()))?;
        let renderer = Arc::new(RenderManager::new(vulkan, config.vsync)?);
        let physics = Arc::new(PhysicsWorld::new(config.physics_timestep));
        let scheduler = Arc::new(TaskScheduler::new(config.thread_count));
        let systems = Arc::new(SystemManager::new());
        let harmony_sim = Arc::new(HarmonySimulator::new(crystal.clone()));

        Ok(Self {
            config,
            crystal,
            renderer,
            physics,
            systems,
            scheduler,
            harmony_sim,
            running: Arc::new(AtomicBool::new(false)),
           frame_count: Arc::new(AtomicU64::new(0)),
           last_frame: Arc::new(Mutex::new(Instant::now())),
           frame_times: Arc::new(RwLock::new(Vec::with_capacity(60))),
        })
    }

    /// Start the engine
    pub fn start(&self) -> EngineResult<()> {
        info!("Starting engine main loop");
        self.running.store(true, Ordering::Release);

        while self.running.load(Ordering::Acquire) {
            self.update()?;
            self.render()?;
            self.maintain_frame_rate();
        }

        Ok(())
    }

    /// Stop the engine
    pub fn stop(&self) {
        info!("Stopping engine");
        self.running.store(false, Ordering::Release);
    }

    /// Update all systems
    fn update(&self) -> EngineResult<()> {
        let frame_start = Instant::now();
        let delta_time = frame_start.duration_since(*self.last_frame.lock());
        *self.last_frame.lock() = frame_start;

        // Update crystal state
        self.crystal.write().stabilize().map_err(EngineError::Crystal)?;

        // Schedule parallel tasks
        self.scheduler.schedule(|| {
            // Update physics
            self.physics.step();

            // Update harmony simulation
            self.harmony_sim.update(delta_time);

            // Update systems
            self.systems.update(delta_time);
        });

        // Wait for tasks to complete
        self.scheduler.wait_all();

        // Update frame metrics
        self.frame_count.fetch_add(1, Ordering::Release);
        self.frame_times.write().push(delta_time);
        if self.frame_times.read().len() > 60 {
            self.frame_times.write().remove(0);
        }

        Ok(())
    }

    /// Render frame
    fn render(&self) -> EngineResult<()> {
        self.renderer.begin_frame()?;
        self.systems.render(&self.renderer)?;
        self.renderer.end_frame()?;
        Ok(())
    }

    /// Maintain target frame rate
    fn maintain_frame_rate(&self) {
        let frame_time = self.last_frame.lock()
        .elapsed();

        if frame_time > self.config.max_frame_time {
            warn!("Frame time exceeded maximum: {:?}", frame_time);
        } else if self.config.vsync {
            std::thread::yield_now();
        }
    }

    /// Get engine metrics
    pub fn metrics(&self) -> EngineMetrics {
        let frame_times = self.frame_times.read();
        let avg_frame_time = if !frame_times.is_empty() {
            frame_times.iter().sum::<Duration>() / frame_times.len() as u32
        } else {
            Duration::default()
        };

        EngineMetrics {
            frame_count: self.frame_count.load(Ordering::Acquire),
            frame_time: avg_frame_time,
            crystal: self.crystal.read().metrics(),
            physics_active_bodies: self.physics.active_body_count(),
            harmony_coherence: self.harmony_sim.coherence(),
        }
    }
}

/// Engine metrics
#[derive(Debug, Clone)]
pub struct EngineMetrics {
    pub frame_count: u64,
    pub frame_time: Duration,
    pub crystal: CrystalMetrics,
    pub physics_active_bodies: usize,
    pub harmony_coherence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let config = EngineConfig::default();
        let engine = Engine::new(config).unwrap();
        assert!(!engine.running.load(Ordering::Acquire));
    }

    #[test]
    fn test_engine_start_stop() {
        let config = EngineConfig::default();
        let engine = Engine::new(config).unwrap();

        std::thread::spawn({
            let engine = engine.clone();
            move || {
                std::thread::sleep(Duration::from_millis(100));
                engine.stop();
            }
        });

        engine.start().unwrap();
        assert!(!engine.running.load(Ordering::Acquire));
    }

    #[test]
    fn test_engine_metrics() {
        let config = EngineConfig::default();
        let engine = Engine::new(config).unwrap();
        let metrics = engine.metrics();

        assert_eq!(metrics.frame_count, 0);
        assert_eq!(metrics.physics_active_bodies, 0);
        assert!(metrics.harmony_coherence > 0.0);
    }
}
