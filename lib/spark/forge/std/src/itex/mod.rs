//! Native iterator extension type with crystal resonance.
//!
//! This module provides a high-performance iterator extension type optimized for
//! crystal-space quantum parallelization and flux operations.

pub mod crystal;
pub mod flux;

use std::iter::Iterator;
use crystal::Crystal;
use flux::Flux;

/// Result type for itex operations
pub type ItexResult<T> = Result<T, ItexError>;

/// Error type for itex operations
#[derive(Debug)]
pub enum ItexError {
    /// Quantum flux error
    FluxError(String),
    /// Crystal resonance error
    ResonanceError(String),
    /// Parallelization error
    ParallelError(String),
}

impl From<String> for ItexError {
    fn from(error: String) -> Self {
        ItexError::FluxError(error)
    }
}

/// Native iterator extension with crystal resonance
#[derive(Debug)]
pub struct Itex<I: Iterator> {
    /// Inner iterator
    inner: I,
    /// Crystal state
    crystal: Crystal,
    /// Quantum flux
    flux: Flux,
}

impl<I: Iterator> Itex<I> {
    /// Creates a new iterator extension
    pub fn new(iter: I) -> Self {
        Self {
            inner: iter,
            crystal: Crystal::default(),
            flux: Flux::default(),
        }
    }

    /// Gets the crystal state
    pub fn crystal(&self) -> &Crystal {
        &self.crystal
    }

    /// Gets the quantum flux
    pub fn flux(&self) -> &Flux {
        &self.flux
    }

    /// Maps elements through crystal resonance
    pub fn crystal_map<B, F>(self, mut f: F) -> Itex<impl Iterator<Item = B>>
    where
        F: FnMut(I::Item) -> B,
    {
        let flux = self.flux;
        let crystal = self.crystal;

        Itex::new(self.inner.map(move |x| {
            crystal.resonate();
            flux.propagate();
            f(x)
        }))
    }

    /// Filters elements through quantum flux
    pub fn flux_filter<P>(self, mut predicate: P) -> Itex<impl Iterator<Item = I::Item>>
    where
        P: FnMut(&I::Item) -> bool,
    {
        let flux = self.flux;
        let crystal = self.crystal;

        Itex::new(self.inner.filter(move |x| {
            crystal.resonate();
            flux.propagate();
            predicate(x)
        }))
    }

    /// Collects elements with quantum acceleration
    pub fn quantum_collect<B>(mut self) -> ItexResult<B>
    where
        B: Default + Extend<I::Item>,
    {
        let mut result = B::default();
        self.crystal.accelerate()?;
        self.flux.stabilize()?;

        while let Some(item) = self.inner.next() {
            result.extend(Some(item));
            self.crystal.resonate();
            self.flux.propagate();
        }

        Ok(result)
    }

    /// Parallel processing with crystal resonance
    pub fn parallel_process<B, F>(mut self, chunks: usize, f: F) -> ItexResult<Vec<B>>
    where
        F: Fn(I::Item) -> B + Send + Sync,
        I::Item: Send + Clone,
        B: Send,
    {
        self.crystal.accelerate()?;
        self.flux.parallelize(chunks)?;

        let items: Vec<_> = self.inner.collect();
        let chunk_size = (items.len() + chunks - 1) / chunks;

        let results: Vec<_> = items
            .chunks(chunk_size)
            .flat_map(|chunk| {
                chunk.iter()
                    .cloned()
                    .map(&f)
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(results)
    }
}

impl<I: Iterator> Iterator for Itex<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.crystal.resonate();
        self.flux.propagate();
        self.inner.next()
    }
}
