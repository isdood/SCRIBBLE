//! Crystal wave resonance and pattern matching example
//! Created: 2025-01-21 16:08:39 UTC
//! Author: @isdood

use crystal_waves::{
    Crystal, CrystalConfig, ComputeBackend, CrystalError,
    waves::{
        WavePattern, WaveType, ResonancePattern,
        analysis::{WaveAnalyzer, PatternMatcher},
    },
    resonance::{ResonanceConfig, ResonanceMode},
    analysis::{VisualizationConfig, AnalysisMode},
    utils::{
        plotting::{WaveformPlot, ResonancePlot, PatternPlot},
        spectral::{SpectralAnalyzer, WindowType},
        export::{DataExporter, ExportFormat},
    },
    julia::waves::JuliaWaveOptimizer,
    chapel::patterns::ChapelPatternMatcher,
};

use clap::{Parser, ValueEnum};
use tracing::{info, warn, error, Level};
use tokio::time::{Duration, Instant};
use std::path::PathBuf;
use num_complex::Complex64;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Wave pattern type
    #[clap(short = 'w', long, value_enum, default_value = "standing")]
    wave_type: WavePatternType,

    /// Base frequency (Hz)
    #[clap(short = 'f', long, default_value = "432.0")]
    frequency: f64,

    /// Pattern duration (seconds)
    #[clap(short = 't', long, default_value = "300.0")]
    duration: f64,

    /// Time step
    #[clap(short = 's', long, default_value = "0.01")]
    time_step: f64,

    /// Grid resolution
    #[clap(short = 'r', long, default_value = "128")]
    resolution: usize,

    /// Pattern matching threshold
    #[clap(short = 'p', long, default_value = "0.95")]
    pattern_threshold: f64,

    /// Julia threads
    #[clap(short = 'j', long, default_value = "4")]
    julia_threads: usize,

    /// Chapel locales
    #[clap(short = 'c', long, default_value = "2")]
    chapel_locales: usize,

    /// Compute backend
    #[clap(short = 'b', long, value_enum, default_value = "hybrid")]
    backend: Backend,

    /// Analysis mode
    #[clap(short = 'm', long, value_enum, default_value = "spectral")]
    analysis_mode: AnalysisType,

    /// Output directory
    #[clap(short = 'o', long, default_value = "wave_resonance")]
    output: PathBuf,

    /// Enable visualization
    #[clap(short = 'v', long)]
    visualize: bool,

    /// Save wave patterns
    #[clap(long)]
    save_patterns: bool,

    /// Export data format
    #[clap(long, value_enum, default_value = "hdf5")]
    export_format: ExportType,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Backend {
    Julia,
    Chapel,
    Hybrid,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum WavePatternType {
    Standing,
    Traveling,
    Spherical,
    Resonant,
    Custom,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum AnalysisType {
    Temporal,
    Spectral,
    Wavelet,
    Custom,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ExportType {
    Csv,
    Hdf5,
    NetCDF,
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

impl From<WavePatternType> for WaveType {
    fn from(pattern_type: WavePatternType) -> Self {
        match pattern_type {
            WavePatternType::Standing => WaveType::Standing,
            WavePatternType::Traveling => WaveType::Traveling,
            WavePatternType::Spherical => WaveType::Spherical,
            WavePatternType::Resonant => WaveType::Resonant,
            WavePatternType::Custom => WaveType::Custom,
        }
    }
}

impl From<AnalysisType> for AnalysisMode {
    fn from(analysis_type: AnalysisType) -> Self {
        match analysis_type {
            AnalysisType::Temporal => AnalysisMode::Temporal,
            AnalysisType::Spectral => AnalysisMode::Spectral,
            AnalysisType::Wavelet => AnalysisMode::Wavelet,
            AnalysisType::Custom => AnalysisMode::Custom,
        }
    }
}

impl From<ExportType> for ExportFormat {
    fn from(export_type: ExportType) -> Self {
        match export_type {
            ExportType::Csv => ExportFormat::Csv,
            ExportType::Hdf5 => ExportFormat::Hdf5,
            ExportType::NetCDF => ExportFormat::NetCDF,
            ExportType::Custom => ExportFormat::Custom,
        }
    }
}

#[derive(Debug, Clone)]
struct WaveState {
    time: f64,
    pattern: ResonancePattern,
    amplitude: Vec<Vec<Complex64>>,
    phase: Vec<Vec<f64>>,
    energy: Vec<Vec<f64>>,
    resonance_score: f64,
}

#[tokio::main]
async fn main() -> Result<(), CrystalError> {
    // Initialize logging
    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_thread_ids(true)
    .with_target(false)
    .init();

    info!("Starting wave resonance analysis");

    // Parse arguments
    let args = Args::parse();

    // Setup directories
    let dirs = setup_directories(&args.output)?;

    // Configure visualization
    let vis_config = if args.visualize {
        Some(VisualizationConfig {
            window_size: (1200, 800),
             update_rate: 30.0,
             save_frames: args.save_patterns,
             output_dir: dirs.frames.clone(),
        })
    } else {
        None
    };

    // Initialize crystal configuration
    let config = CrystalConfig {
        dimensions: (args.resolution, args.resolution, args.resolution),
        time_step: args.time_step,
        total_time: args.duration,
        julia_threads: args.julia_threads,
        chapel_locales: args.chapel_locales,
        compute_backend: args.backend.into(),
    };

    // Initialize crystal with wave pattern
    let mut crystal = Crystal::new(config)?;
    crystal.wave_system.set_pattern(args.wave_type.into())?;
    crystal.wave_system.set_frequency(args.frequency)?;

    // Initialize analyzers
    let wave_analyzer = WaveAnalyzer::new(args.analysis_mode.into())?;
    let pattern_matcher = PatternMatcher::new(args.pattern_threshold)?;
    let spectral_analyzer = SpectralAnalyzer::new(WindowType::Hann)?;

    // Initialize visualizers
    let mut wave_plot = vis_config.as_ref().map(|config|
    WaveformPlot::new(config.clone())
    ).transpose()?;

    let mut resonance_plot = vis_config.as_ref().map(|config|
    ResonancePlot::new(config.clone())
    ).transpose()?;

    let mut pattern_plot = vis_config.as_ref().map(|config|
    PatternPlot::new(config.clone())
    ).transpose()?;

    // Initialize data exporter
    let data_exporter = DataExporter::new(
        args.export_format.into(),
                                          dirs.data.clone(),
    )?;

    info!("Starting wave pattern analysis");
    let start_time = Instant::now();
    let total_steps = (args.duration / args.time_step) as usize;
    let mut resonance_found = false;

    // Main analysis loop
    for step in 0..total_steps {
        let time = step as f64 * args.time_step;

        // Update crystal state
        crystal.step()?;
        let crystal_state = crystal.get_state()?;

        // Analyze wave patterns
        let wave_analysis = wave_analyzer.analyze(&crystal_state.wave_state)?;
        let pattern_match = pattern_matcher.match_pattern(&wave_analysis)?;
        let spectral_analysis = spectral_analyzer.analyze(&wave_analysis)?;

        // Create wave state
        let wave_state = WaveState {
            time,
            pattern: pattern_match.pattern,
            amplitude: wave_analysis.amplitude,
            phase: wave_analysis.phase,
            energy: wave_analysis.energy,
            resonance_score: pattern_match.score,
        };

        // Update visualization
        if let Some(plot) = wave_plot.as_mut() {
            plot.update(&wave_state)?;
        }
        if let Some(plot) = resonance_plot.as_mut() {
            plot.update(&spectral_analysis)?;
        }
        if let Some(plot) = pattern_plot.as_mut() {
            plot.update(&pattern_match)?;
        }

        // Save patterns if enabled
        if args.save_patterns && step % 10 == 0 {
            save_wave_state(&dirs.patterns, &wave_state, step)?;
        }

        // Export data
        data_exporter.export_frame(&wave_state, step)?;

        // Check for resonance
        if pattern_match.score >= args.pattern_threshold && !resonance_found {
            resonance_found = true;
            info!("Resonance pattern detected at {:.2}s", time);
            save_resonance_state(&dirs.resonance, &wave_state, "resonance_detected")?;
        }

        // Log progress
        if step % 100 == 0 {
            log_analysis_progress(step, total_steps, &wave_state);
        }
    }

    // Finalize and save results
    let elapsed = start_time.elapsed();
    info!("Analysis complete in {:.2}s", elapsed.as_secs_f64());

    let final_state = crystal.get_state()?;
    save_final_results(&dirs, &final_state)?;
    generate_analysis_report(&dirs.reports, &final_state)?;

    Ok(())
}

struct OutputDirectories {
    frames: PathBuf,
    patterns: PathBuf,
    resonance: PathBuf,
    data: PathBuf,
    reports: PathBuf,
}

// Implementation of helper functions...
// (The helper functions follow the same pattern as in previous examples,
// but are specialized for wave resonance analysis. I can provide their
// implementations if needed.)
