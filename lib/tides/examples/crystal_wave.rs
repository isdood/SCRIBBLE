//! Crystal wave simulation example
//! Created: 2025-01-21 16:01:43 UTC
//! Author: @isdood

use crystal_waves::{
    Crystal, CrystalConfig, ComputeBackend, CrystalError,
    waves::{WavePattern, WaveType},
    analysis::VisualizationConfig,
    utils::plotting::{Plot3D, ColorMap},
};

use clap::{Parser, ValueEnum};
use tracing::{info, warn, error};
use std::time::Instant;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Size of the crystal grid (N x N x N)
    #[clap(short, long, default_value = "64")]
    size: usize,

    /// Total simulation time
    #[clap(short, long, default_value = "10.0")]
    time: f64,

    /// Time step for simulation
    #[clap(short = 's', long, default_value = "0.01")]
    time_step: f64,

    /// Number of Julia threads
    #[clap(short = 'j', long, default_value = "4")]
    julia_threads: usize,

    /// Number of Chapel locales
    #[clap(short = 'c', long, default_value = "2")]
    chapel_locales: usize,

    /// Compute backend to use
    #[clap(short = 'b', long, value_enum, default_value = "hybrid")]
    backend: Backend,

    /// Wave pattern type
    #[clap(short = 'w', long, value_enum, default_value = "standing")]
    wave_type: WavePatternType,

    /// Output directory for results
    #[clap(short, long, default_value = "results")]
    output: PathBuf,

    /// Enable real-time visualization
    #[clap(short = 'v', long)]
    visualize: bool,

    /// Save simulation frames
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
enum WavePatternType {
    Standing,
    Traveling,
    Spherical,
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
    fn from(wave_type: WavePatternType) -> Self {
        match wave_type {
            WavePatternType::Standing => WaveType::Standing,
            WavePatternType::Traveling => WaveType::Traveling,
            WavePatternType::Spherical => WaveType::Spherical,
            WavePatternType::Custom => WaveType::Custom,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), CrystalError> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting crystal wave simulation");

    // Parse command line arguments
    let args = Args::parse();

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output)?;

    // Configure visualization
    let vis_config = if args.visualize {
        Some(VisualizationConfig {
            window_size: (800, 600),
             colormap: ColorMap::Viridis,
             save_frames: args.save_frames,
             output_dir: args.output.clone(),
        })
    } else {
        None
    };

    // Create crystal configuration
    let config = CrystalConfig {
        dimensions: (args.size, args.size, args.size),
        time_step: args.time_step,
        total_time: args.time,
        julia_threads: args.julia_threads,
        chapel_locales: args.chapel_locales,
        compute_backend: args.backend.into(),
    };

    // Initialize crystal simulation
    info!("Initializing crystal simulation with {:?} backend", args.backend);
    let mut crystal = Crystal::new(config)?;

    // Set up wave pattern
    let wave_type: WaveType = args.wave_type.into();
    crystal.wave_system.set_pattern(wave_type)?;

    // Initialize simulation
    crystal.initialize()?;
    info!("Initialization complete");

    // Set up visualization if enabled
    let mut plotter = if args.visualize {
        Some(Plot3D::new(vis_config.unwrap())?)
    } else {
        None
    };

    // Run simulation
    let start_time = Instant::now();
    let total_steps = (args.time / args.time_step) as usize;
    info!("Starting simulation for {} steps", total_steps);

    for step in 0..total_steps {
        // Run simulation step
        crystal.step()?;

        // Get current state
        let state = crystal.get_state()?;

        // Update visualization if enabled
        if let Some(plotter) = plotter.as_mut() {
            plotter.update(&state)?;
        }

        // Log progress
        if step % 100 == 0 {
            let progress = (step as f64 / total_steps as f64) * 100.0;
            info!("Simulation progress: {:.1}%", progress);
        }

        // Save state periodically
        if args.save_frames && step % 10 == 0 {
            save_state(&args.output, &state, step)?;
        }
    }

    // Calculate and log performance metrics
    let elapsed = start_time.elapsed();
    let steps_per_second = total_steps as f64 / elapsed.as_secs_f64();
    info!("Simulation complete");
    info!("Total time: {:.2} seconds", elapsed.as_secs_f64());
    info!("Steps per second: {:.2}", steps_per_second);

    // Save final state
    let final_state = crystal.get_state()?;
    save_final_results(&args.output, &final_state)?;

    // Generate analysis report
    generate_report(&args.output, &final_state)?;

    Ok(())
}

/// Save simulation state to file
fn save_state(
    output_dir: &PathBuf,
    state: &crystal_waves::SimulationState,
    step: usize,
) -> Result<(), CrystalError> {
    let frame_dir = output_dir.join("frames");
    std::fs::create_dir_all(&frame_dir)?;

    let filename = frame_dir.join(format!("state_{:06}.json", step));
    let file = std::fs::File::create(filename)?;
    serde_json::to_writer_pretty(file, &state)?;

    Ok(())
}

/// Save final simulation results
fn save_final_results(
    output_dir: &PathBuf,
    state: &crystal_waves::SimulationState,
) -> Result<(), CrystalError> {
    // Save final state
    let state_file = output_dir.join("final_state.json");
    let file = std::fs::File::create(state_file)?;
    serde_json::to_writer_pretty(file, &state)?;

    // Save wave patterns
    let wave_file = output_dir.join("wave_patterns.npy");
    save_numpy_array(&wave_file, &state.wave_state.amplitudes)?;

    // Save energy distribution
    let energy_file = output_dir.join("energy_distribution.npy");
    save_numpy_array(&energy_file, &state.resonance_state.energy_distribution)?;

    // Save mesh data
    let mesh_file = output_dir.join("mesh.vtk");
    save_vtk_mesh(&mesh_file, &state.mesh_state)?;

    Ok(())
}

/// Generate analysis report
fn generate_report(
    output_dir: &PathBuf,
    state: &crystal_waves::SimulationState,
) -> Result<(), CrystalError> {
    let report_file = output_dir.join("analysis_report.md");
    let mut report = String::new();

    // Add report header
    report.push_str(&format!("# Crystal Wave Simulation Analysis Report\n"));
    report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now()));

    // Add wave analysis
    report.push_str("## Wave Analysis\n");
    report.push_str(&format!("- Total Energy: {:.6}\n", state.wave_state.total_energy));
    report.push_str(&format!("- Phase Coherence: {:.6}\n", state.wave_state.phase_coherence));
    report.push_str(&format!("- Wave Amplitude Range: [{:.6}, {:.6}]\n\n",
                             state.wave_state.amplitude_range.0,
                             state.wave_state.amplitude_range.1));

    // Add resonance analysis
    report.push_str("## Resonance Analysis\n");
    report.push_str(&format!("- Number of Resonant Modes: {}\n", state.resonance_state.modes.len()));
    report.push_str(&format!("- Resonance Stability: {:.6}\n", state.resonance_state.stability));
    report.push_str(&format!("- Energy Conservation: {:.6}\n\n", state.resonance_state.energy_conservation));

    // Add mesh analysis
    report.push_str("## Mesh Analysis\n");
    report.push_str(&format!("- Number of Vertices: {}\n", state.mesh_state.vertices.len()));
    report.push_str(&format!("- Number of Elements: {}\n", state.mesh_state.elements.len()));
    report.push_str(&format!("- Mesh Quality: {:.6}\n\n", state.mesh_state.quality_metrics.overall_quality));

    // Add analysis results
    report.push_str("## Analysis Results\n");
    for (metric, value) in &state.analysis.metrics {
        report.push_str(&format!("- {}: {:.6}\n", metric, value));
    }

    // Save report
    std::fs::write(report_file, report)?;

    Ok(())
}

/// Save array as numpy file
fn save_numpy_array<T>(filename: &PathBuf, data: &[T]) -> Result<(), CrystalError> {
    use ndarray_npy::write_npy;
    use ndarray::Array;

    let array = Array::from_vec(data.to_vec());
    write_npy(filename, &array)?;
    Ok(())
}

/// Save mesh in VTK format
fn save_vtk_mesh(filename: &PathBuf, mesh_state: &crystal_waves::mesh::MeshState) -> Result<(), CrystalError> {
    use vtkio::model::*;

    // Create VTK data structure
    let mut vtk = Vtk::new(Version::new((4, 2)), VtkType::UnstructuredGrid);

    // Add vertices
    let points = mesh_state.vertices.iter()
    .map(|&[x, y, z]| vec![x, y, z])
    .collect();
    vtk.set_points(points);

    // Add elements
    let cells = mesh_state.elements.iter()
    .map(|&[a, b, c, d]| vec![a as u32, b as u32, c as u32, d as u32])
    .collect();
    vtk.set_cells(cells);

    // Write to file
    vtk.write_xml(filename)?;
    Ok(())
}
