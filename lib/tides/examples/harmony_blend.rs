//! Crystal harmonic blending and resonance optimization example
//! Created: 2025-01-21 16:06:14 UTC
//! Author: @isdood

use crystal_waves::{
    Crystal, CrystalConfig, ComputeBackend, CrystalError,
    resonance::{HarmonicBlender, BlendMode, ResonanceProfile},
    waves::{WavePattern, WaveType, FrequencyRange},
    analysis::{HarmonicAnalyzer, VisualizationConfig},
    utils::{
        plotting::{HarmonicSpectrum, ResonancePlot, BlendVisualizer},
        audio::{HarmonicSynth, AudioConfig},
    },
    julia::harmonics::JuliaHarmonicOptimizer,
    chapel::resonance::ChapelResonanceBlender,
};

use clap::{Parser, ValueEnum};
use tracing::{info, warn, error, Level};
use tokio::time::{Duration, Instant};
use std::path::PathBuf;
use num_complex::Complex64;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Primary resonance frequency (Hz)
    #[clap(short, long, default_value = "432.0")]
    primary_freq: f64,

    /// Secondary resonance frequency (Hz)
    #[clap(short, long, default_value = "528.0")]
    secondary_freq: f64,

    /// Harmonic depth for blending
    #[clap(short = 'd', long, default_value = "12")]
    harmonic_depth: usize,

    /// Blend duration (seconds)
    #[clap(short = 't', long, default_value = "600.0")]
    duration: f64,

    /// Time step for blending
    #[clap(short = 's', long, default_value = "0.01")]
    time_step: f64,

    /// Blend coherence threshold
    #[clap(short = 'c', long, default_value = "0.98")]
    coherence_threshold: f64,

    /// Number of Julia threads
    #[clap(short = 'j', long, default_value = "4")]
    julia_threads: usize,

    /// Number of Chapel locales
    #[clap(short = 'l', long, default_value = "2")]
    chapel_locales: usize,

    /// Compute backend
    #[clap(short = 'b', long, value_enum, default_value = "hybrid")]
    backend: Backend,

    /// Harmonic blend mode
    #[clap(short = 'm', long, value_enum, default_value = "adaptive")]
    blend_mode: BlendType,

    /// Output directory
    #[clap(short = 'o', long, default_value = "harmony_blend")]
    output: PathBuf,

    /// Enable visualization
    #[clap(short = 'v', long)]
    visualize: bool,

    /// Enable audio synthesis
    #[clap(short = 'a', long)]
    audio: bool,

    /// Save blend states
    #[clap(short = 'f', long)]
    save_states: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Backend {
    Julia,
    Chapel,
    Hybrid,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum BlendType {
    Linear,
    Exponential,
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

impl From<BlendType> for BlendMode {
    fn from(mode: BlendType) -> Self {
        match mode {
            BlendType::Linear => BlendMode::Linear,
            BlendType::Exponential => BlendMode::Exponential,
            BlendType::Adaptive => BlendMode::Adaptive,
            BlendType::Custom => BlendMode::Custom,
        }
    }
}

struct BlendState {
    time: f64,
    primary_harmonics: Vec<f64>,
    secondary_harmonics: Vec<f64>,
    blend_harmonics: Vec<f64>,
    coherence: f64,
    energy: f64,
    stability: f64,
}

#[tokio::main]
async fn main() -> Result<(), CrystalError> {
    // Initialize logging
    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_thread_ids(true)
    .with_target(false)
    .init();

    info!("Starting harmonic blending process");

    // Parse arguments
    let args = Args::parse();

    // Setup directories
    let dirs = setup_directories(&args.output)?;

    // Configure visualization
    let vis_config = if args.visualize {
        Some(VisualizationConfig {
            window_size: (1600, 900),
             update_rate: 30.0,
             save_frames: args.save_states,
             output_dir: dirs.frames.clone(),
        })
    } else {
        None
    };

    // Configure audio
    let audio_config = if args.audio {
        Some(AudioConfig {
            sample_rate: 48000,
            buffer_size: 1024,
            output_dir: dirs.audio.clone(),
        })
    } else {
        None
    };

    // Initialize harmonic blender
    let blender = create_blender(&args)?;

    // Initialize visualizers
    let mut spectrum_vis = vis_config.as_ref().map(|config|
    HarmonicSpectrum::new(config.clone())
    ).transpose()?;

    let mut resonance_vis = vis_config.as_ref().map(|config|
    ResonancePlot::new(config.clone())
    ).transpose()?;

    let mut blend_vis = vis_config.as_ref().map(|config|
    BlendVisualizer::new(config.clone())
    ).transpose()?;

    // Initialize audio synthesizer
    let mut synth = audio_config.as_ref().map(|config|
    HarmonicSynth::new(config.clone())
    ).transpose()?;

    // Create resonance profiles
    let primary_profile = ResonanceProfile::new(
        args.primary_freq,
        args.harmonic_depth,
        FrequencyRange::new(20.0, 20000.0),
    )?;

    let secondary_profile = ResonanceProfile::new(
        args.secondary_freq,
        args.harmonic_depth,
        FrequencyRange::new(20.0, 20000.0),
    )?;

    // Initialize analyzers
    let harmonic_analyzer = HarmonicAnalyzer::new(args.backend.into())?;

    info!("Starting blend process with {} mode", args.blend_mode);
    let start_time = Instant::now();
    let total_steps = (args.duration / args.time_step) as usize;
    let mut blend_complete = false;

    // Main blend loop
    for step in 0..total_steps {
        let time = step as f64 * args.time_step;

        // Compute blend state
        let blend_state = blender.compute_blend(
            &primary_profile,
            &secondary_profile,
            time / args.duration,
        )?;

        // Analyze harmonics
        let analysis = harmonic_analyzer.analyze(&blend_state)?;

        // Update visualization
        if let Some(vis) = spectrum_vis.as_mut() {
            vis.update(&analysis.spectrum)?;
        }
        if let Some(vis) = resonance_vis.as_mut() {
            vis.update(&analysis.resonances)?;
        }
        if let Some(vis) = blend_vis.as_mut() {
            vis.update(&blend_state)?;
        }

        // Update audio
        if let Some(synth) = synth.as_mut() {
            synth.process_frame(&blend_state)?;
        }

        // Save state if enabled
        if args.save_states && step % 10 == 0 {
            save_blend_state(&dirs.states, &blend_state, step)?;
        }

        // Check for blend completion
        if analysis.coherence >= args.coherence_threshold && !blend_complete {
            blend_complete = true;
            info!("Blend coherence threshold reached at {:.2}s", time);
            save_checkpoint(&dirs.checkpoints, &blend_state, "coherence_reached")?;
        }

        // Log progress
        if step % 100 == 0 {
            log_blend_progress(step, total_steps, &analysis);
        }
    }

    // Finalize and save results
    let elapsed = start_time.elapsed();
    info!("Blend process complete in {:.2}s", elapsed.as_secs_f64());

    let final_state = blender.get_final_state()?;
    save_final_results(&dirs, &final_state)?;
    generate_blend_report(&dirs.reports, &final_state)?;

    Ok(())
}

struct OutputDirectories {
    frames: PathBuf,
    audio: PathBuf,
    states: PathBuf,
    checkpoints: PathBuf,
    reports: PathBuf,
}

fn setup_directories(base_dir: &PathBuf) -> Result<OutputDirectories, CrystalError> {
    let dirs = OutputDirectories {
        frames: base_dir.join("frames"),
        audio: base_dir.join("audio"),
        states: base_dir.join("states"),
        checkpoints: base_dir.join("checkpoints"),
        reports: base_dir.join("reports"),
    };

    for dir in [&dirs.frames, &dirs.audio, &dirs.states, &dirs.checkpoints, &dirs.reports] {
        std::fs::create_dir_all(dir)?;
    }

    Ok(dirs)
}

fn create_blender(args: &Args) -> Result<HarmonicBlender, CrystalError> {
    let config = CrystalConfig {
        dimensions: (128, 128, 128),
        time_step: args.time_step,
        total_time: args.duration,
        julia_threads: args.julia_threads,
        chapel_locales: args.chapel_locales,
        compute_backend: args.backend.into(),
    };

    HarmonicBlender::new(config, args.blend_mode.into())
}

fn save_blend_state(
    output_dir: &PathBuf,
    state: &BlendState,
    step: usize,
) -> Result<(), CrystalError> {
    let filename = output_dir.join(format!("blend_state_{:06}.json", step));
    let file = std::fs::File::create(filename)?;
    serde_json::to_writer_pretty(file, &state)?;
    Ok(())
}

fn save_checkpoint(
    output_dir: &PathBuf,
    state: &BlendState,
    label: &str,
) -> Result<(), CrystalError> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = output_dir.join(format!("{}_{}.json", label, timestamp));
    let file = std::fs::File::create(filename)?;
    serde_json::to_writer_pretty(file, &state)?;
    Ok(())
}

fn log_blend_progress(step: usize, total_steps: usize, analysis: &HarmonicAnalyzer) {
    let progress = (step as f64 / total_steps as f64) * 100.0;
    info!(
        "Progress: {:.1}% - Coherence: {:.4} - Energy: {:.4} - Stability: {:.4}",
        progress,
        analysis.coherence,
        analysis.energy,
        analysis.stability,
    );
}

fn save_final_results(
    dirs: &OutputDirectories,
    state: &BlendState,
) -> Result<(), CrystalError> {
    // Save harmonic data
    let harmonics_file = dirs.reports.join("final_harmonics.npy");
    save_numpy_array(&harmonics_file, &state.blend_harmonics)?;

    // Save analysis data
    let analysis_file = dirs.reports.join("final_analysis.json");
    let file = std::fs::File::create(analysis_file)?;
    serde_json::to_writer_pretty(file, &state)?;

    Ok(())
}

fn generate_blend_report(
    output_dir: &PathBuf,
    state: &BlendState,
) -> Result<(), CrystalError> {
    let report_file = output_dir.join("blend_report.md");
    let mut report = String::new();

    report.push_str("# Harmonic Blend Analysis Report\n");
    report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now()));

    // Add harmonic analysis
    report.push_str("## Harmonic Analysis\n");
    report.push_str(&format!("- Coherence: {:.6}\n", state.coherence));
    report.push_str(&format!("- Energy: {:.6}\n", state.energy));
    report.push_str(&format!("- Stability: {:.6}\n\n", state.stability));

    // Add harmonic comparisons
    report.push_str("## Harmonic Comparisons\n");
    for i in 0..state.primary_harmonics.len() {
        report.push_str(&format!(
            "Harmonic {}: Primary = {:.2} Hz, Secondary = {:.2} Hz, Blend = {:.2} Hz\n",
            i + 1,
            state.primary_harmonics[i],
            state.secondary_harmonics[i],
            state.blend_harmonics[i],
        ));
    }

    std::fs::write(report_file, report)?;
    Ok(())
}

fn save_numpy_array<T: Clone>(filename: &PathBuf, data: &[T]) -> Result<(), CrystalError> {
    use ndarray_npy::write_npy;
    use ndarray::Array;

    let array = Array::from_vec(data.to_vec());
    write_npy(filename, &array)?;
    Ok(())
}
