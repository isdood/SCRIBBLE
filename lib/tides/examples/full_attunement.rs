//! Full crystal attunement and harmonization example
//! Created: 2025-01-21 16:03:55 UTC
//! Author: @isdood

use crystal_waves::{
    Crystal, CrystalConfig, ComputeBackend, CrystalError,
    resonance::{ResonanceConfig, AttunementMode},
    waves::{WavePattern, WaveType},
    analysis::{VisualizationConfig, AnalysisMode},
    utils::{
        plotting::{SpectralPlot, HarmonicPlot, ColorMap},
        audio::{AudioOutput, WaveformType},
    },
};

use clap::{Parser, ValueEnum};
use tracing::{info, warn, error, Level};
use tokio::time::{Duration, Instant};
use std::path::PathBuf;
use num_complex::Complex64;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Base frequency for attunement (Hz)
    #[clap(short, long, default_value = "432.0")]
    frequency: f64,

    /// Number of harmonic levels
    #[clap(short = 'n', long, default_value = "7")]
    harmonic_levels: usize,

    /// Attunement duration (seconds)
    #[clap(short, long, default_value = "300.0")]
    duration: f64,

    /// Time step for attunement
    #[clap(short = 's', long, default_value = "0.01")]
    time_step: f64,

    /// Phase coherence threshold
    #[clap(short = 'p', long, default_value = "0.95")]
    phase_threshold: f64,

    /// Number of Julia threads
    #[clap(short = 'j', long, default_value = "4")]
    julia_threads: usize,

    /// Number of Chapel locales
    #[clap(short = 'c', long, default_value = "2")]
    chapel_locales: usize,

    /// Compute backend to use
    #[clap(short = 'b', long, value_enum, default_value = "hybrid")]
    backend: Backend,

    /// Attunement mode
    #[clap(short = 'm', long, value_enum, default_value = "harmonic")]
    mode: AttunementType,

    /// Output directory for results
    #[clap(short, long, default_value = "attunement_results")]
    output: PathBuf,

    /// Enable real-time visualization
    #[clap(short = 'v', long)]
    visualize: bool,

    /// Enable audio output
    #[clap(short = 'a', long)]
    audio: bool,

    /// Save attunement frames
    #[clap(short = 'f', long)]
    save_frames: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Backend {
    Julia,
    Chapel,
    Hybrid,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum AttunementType {
    Harmonic,
    Resonant,
    Adaptive,
    Custom,
}

impl From<Backend> for ComputeBackend {
    fn from(backend: Backend) -> Self {
        match backend {
            Backend::Julia => ComputeBackend::Julia,
            Backend::Chapel => ComputeBackend::Chapel,
            Backend::Hybrid => ComputeBackend::Hybrid,
        }
    }
}

impl From<AttunementType> for AttunementMode {
    fn from(mode: AttunementType) -> Self {
        match mode {
            AttunementType::Harmonic => AttunementMode::Harmonic,
            AttunementType::Resonant => AttunementMode::Resonant,
            AttunementType::Adaptive => AttunementMode::Adaptive,
            AttunementType::Custom => AttunementMode::Custom,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), CrystalError> {
    // Initialize logging with detailed format
    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_thread_ids(true)
    .with_target(false)
    .init();

    info!("Starting full crystal attunement process");

    // Parse command line arguments
    let args = Args::parse();

    // Create output directory structure
    let output_dir = setup_output_directories(&args.output)?;

    // Configure visualization
    let vis_config = if args.visualize {
        Some(VisualizationConfig {
            window_size: (1200, 800),
             colormap: ColorMap::Magma,
             save_frames: args.save_frames,
             output_dir: output_dir.frames.clone(),
        })
    } else {
        None
    };

    // Configure audio output
    let audio_output = if args.audio {
        Some(AudioOutput::new(
            44100, // Sample rate
            WaveformType::Sine,
            output_dir.audio.clone(),
        )?)
    } else {
        None
    };

    // Create crystal configuration
    let config = CrystalConfig {
        dimensions: (128, 128, 128),
        time_step: args.time_step,
        total_time: args.duration,
        julia_threads: args.julia_threads,
        chapel_locales: args.chapel_locales,
        compute_backend: args.backend.into(),
    };

    // Initialize crystal with attunement settings
    info!("Initializing crystal with {} attunement mode", args.mode);
    let mut crystal = Crystal::new(config)?;

    // Configure resonance system
    crystal.resonance.configure(ResonanceConfig {
        base_frequency: args.frequency,
        harmonic_depth: args.harmonic_levels,
        phase_tolerance: args.phase_threshold,
        attunement_mode: args.mode.into(),
                                ..Default::default()
    })?;

    // Initialize visualization components
    let mut spectral_plot = if args.visualize {
        Some(SpectralPlot::new(vis_config.clone().unwrap())?)
    } else {
        None
    };

    let mut harmonic_plot = if args.visualize {
        Some(HarmonicPlot::new(vis_config.unwrap())?)
    } else {
        None
    };

    // Initialize attunement process
    crystal.initialize()?;
    info!("Initialization complete, beginning attunement process");

    let start_time = Instant::now();
    let total_steps = (args.duration / args.time_step) as usize;
    let mut attunement_complete = false;

    // Main attunement loop
    for step in 0..total_steps {
        // Perform attunement step
        crystal.step()?;

        // Get current state
        let state = crystal.get_state()?;

        // Update visualization
        if let Some(plot) = spectral_plot.as_mut() {
            plot.update(&state.resonance_state.frequency_spectrum)?;
        }
        if let Some(plot) = harmonic_plot.as_mut() {
            plot.update(&state.resonance_state.harmonics)?;
        }

        // Generate audio output
        if let Some(audio) = audio_output.as_ref() {
            audio.process_frame(&state.resonance_state)?;
        }

        // Save frame if enabled
        if args.save_frames && step % 10 == 0 {
            save_attunement_frame(&output_dir.frames, &state, step)?;
        }

        // Check attunement progress
        let coherence = state.resonance_state.phase_coherence;
        if coherence >= args.phase_threshold && !attunement_complete {
            attunement_complete = true;
            info!("Attunement threshold reached at step {}", step);
            save_attunement_state(&output_dir.checkpoints, &state, "threshold_reached")?;
        }

        // Log progress
        if step % 100 == 0 {
            log_attunement_progress(step, total_steps, &state);
        }
    }

    // Calculate final metrics
    let elapsed = start_time.elapsed();
    info!("Attunement process complete");
    info!("Total time: {:.2} seconds", elapsed.as_secs_f64());

    // Save final results
    let final_state = crystal.get_state()?;
    save_final_results(&output_dir, &final_state)?;

    // Generate comprehensive report
    generate_attunement_report(&output_dir.reports, &final_state)?;

    Ok(())
}

/// Output directory structure
struct OutputDirectories {
    frames: PathBuf,
    audio: PathBuf,
    checkpoints: PathBuf,
    reports: PathBuf,
}

/// Set up output directory structure
fn setup_output_directories(base_dir: &PathBuf) -> Result<OutputDirectories, CrystalError> {
    let dirs = OutputDirectories {
        frames: base_dir.join("frames"),
        audio: base_dir.join("audio"),
        checkpoints: base_dir.join("checkpoints"),
        reports: base_dir.join("reports"),
    };

    for dir in [&dirs.frames, &dirs.audio, &dirs.checkpoints, &dirs.reports] {
        std::fs::create_dir_all(dir)?;
    }

    Ok(dirs)
}

/// Save attunement frame
fn save_attunement_frame(
    output_dir: &PathBuf,
    state: &crystal_waves::SimulationState,
    step: usize,
) -> Result<(), CrystalError> {
    let filename = output_dir.join(format!("attunement_{:06}.json", step));
    let file = std::fs::File::create(filename)?;
    serde_json::to_writer_pretty(file, &state.resonance_state)?;
    Ok(())
}

/// Save attunement state checkpoint
fn save_attunement_state(
    output_dir: &PathBuf,
    state: &crystal_waves::SimulationState,
    label: &str,
) -> Result<(), CrystalError> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = output_dir.join(format!("{}_{}.json", label, timestamp));
    let file = std::fs::File::create(filename)?;
    serde_json::to_writer_pretty(file, state)?;
    Ok(())
}

/// Log attunement progress
fn log_attunement_progress(step: usize, total_steps: usize, state: &crystal_waves::SimulationState) {
    let progress = (step as f64 / total_steps as f64) * 100.0;
    info!(
        "Progress: {:.1}% - Coherence: {:.4} - Energy: {:.4} - Harmonics: {}",
        progress,
        state.resonance_state.phase_coherence,
        state.resonance_state.total_energy,
        state.resonance_state.harmonics.len(),
    );
}

/// Save final results
fn save_final_results(
    output_dir: &OutputDirectories,
    state: &crystal_waves::SimulationState,
) -> Result<(), CrystalError> {
    // Save harmonic data
    let harmonics_file = output_dir.reports.join("final_harmonics.npy");
    save_numpy_array(&harmonics_file, &state.resonance_state.harmonics)?;

    // Save frequency spectrum
    let spectrum_file = output_dir.reports.join("final_spectrum.npy");
    save_numpy_array(&spectrum_file, &state.resonance_state.frequency_spectrum)?;

    // Save energy distribution
    let energy_file = output_dir.reports.join("final_energy.npy");
    save_numpy_array(&energy_file, &state.resonance_state.energy_distribution)?;

    Ok(())
}

/// Generate attunement report
fn generate_attunement_report(
    output_dir: &PathBuf,
    state: &crystal_waves::SimulationState,
) -> Result<(), CrystalError> {
    let report_file = output_dir.join("attunement_report.md");
    let mut report = String::new();

    // Add report header
    report.push_str("# Crystal Attunement Analysis Report\n");
    report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now()));

    // Add resonance analysis
    report.push_str("## Resonance Analysis\n");
    report.push_str(&format!("- Phase Coherence: {:.6}\n", state.resonance_state.phase_coherence));
    report.push_str(&format!("- Total Energy: {:.6}\n", state.resonance_state.total_energy));
    report.push_str(&format!("- Stability: {:.6}\n\n", state.resonance_state.stability));

    // Add harmonic analysis
    report.push_str("## Harmonic Analysis\n");
    for (i, harmonic) in state.resonance_state.harmonics.iter().enumerate() {
        report.push_str(&format!(
            "- Harmonic {}: Frequency = {:.2} Hz, Amplitude = {:.4}, Phase = {:.4}\n",
            i + 1,
            harmonic.frequency,
            harmonic.amplitude,
            harmonic.phase
        ));
    }

    // Add energy distribution analysis
    report.push_str("\n## Energy Distribution\n");
    let energy_stats = compute_energy_statistics(&state.resonance_state.energy_distribution);
    report.push_str(&format!("- Mean Energy: {:.6}\n", energy_stats.mean));
    report.push_str(&format!("- Energy Variance: {:.6}\n", energy_stats.variance));
    report.push_str(&format!("- Energy Range: [{:.6}, {:.6}]\n", energy_stats.min, energy_stats.max));

    // Save report
    std::fs::write(report_file, report)?;

    Ok(())
}

/// Energy statistics
struct EnergyStatistics {
    mean: f64,
    variance: f64,
    min: f64,
    max: f64,
}

/// Compute energy statistics
fn compute_energy_statistics(energy: &[f64]) -> EnergyStatistics {
    let len = energy.len() as f64;
    let mean = energy.iter().sum::<f64>() / len;
    let variance = energy.iter()
    .map(|&x| (x - mean).powi(2))
    .sum::<f64>() / len;
    let min = energy.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = energy.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    EnergyStatistics {
        mean,
        variance,
        min,
        max,
    }
}

/// Save array as numpy file
fn save_numpy_array<T: Clone>(filename: &PathBuf, data: &[T]) -> Result<(), CrystalError> {
    use ndarray_npy::write_npy;
    use ndarray::Array;

    let array = Array::from_vec(data.to_vec());
    write_npy(filename, &array)?;
    Ok(())
}
