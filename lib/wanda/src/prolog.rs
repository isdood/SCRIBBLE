use std::sync::Arc;
use once_cell::sync::Lazy;
use swipl::prelude::*;
use crate::brain::{WandaBrain, NeuralPattern, BrainState};

static PROLOG_ENGINE: Lazy<Arc<Engine>> = Lazy::new(|| {
    Arc::new(Engine::new().expect("Failed to initialize SWI-Prolog engine"))
});

#[derive(Debug)]
pub struct PrologBridge {
    engine: Arc<Engine>,
    context: Context,
}

impl PrologBridge {
    pub fn new() -> Self {
        let engine = PROLOG_ENGINE.clone();
        let context = Context::new(&engine).expect("Failed to create Prolog context");

        Self { engine, context }
    }

    pub fn init_quantum_rules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rules = r#"
        % Quantum state validation rules
        valid_quantum_state(State, Coherence) :-
        coherence_threshold(Threshold),
        Coherence >= Threshold,
        stable_state(State).

        % Default coherence threshold
        coherence_threshold(0.75).

        % Stable state definition
        stable_state(initializing).
        stable_state(learning).
        stable_state(processing).
        stable_state(resting).
        stable_state(State) :- \+ decoherent(State).

        % Neural pattern validation
        valid_neural_pattern(Confidence, Phase) :-
        quantum_stable(Phase),
        confidence_sufficient(Confidence).

        % Quantum stability check
        quantum_stable(Phase) :-
        Phase >= 0.0,
        Phase =< 1.0.

        % Confidence threshold
        confidence_sufficient(Confidence) :-
        Confidence >= 0.75.

        % Quantum phase alignment
        phase_aligned(Phase, Threshold) :-
        Phase >= 0.0,
        Phase =< 1.0,
        Phase >= Threshold.

        % Pattern coherence validation
        pattern_coherent(Pattern, Coherence) :-
        valid_neural_pattern(Pattern, _),
        Coherence >= 0.75.
        "#;

        self.context.consult_string(rules)?;
        Ok(())
    }

    pub fn query_state(&self, state: &BrainState, coherence: f64) -> Result<bool, Box<dyn std::error::Error>> {
        let state_name = match state {
            BrainState::Initializing => "initializing",
            BrainState::Learning => "learning",
            BrainState::Processing => "processing",
            BrainState::Resting => "resting",
            BrainState::Decoherent => "decoherent",
        };

        let query = format!("valid_quantum_state({}, {}).", state_name, coherence);
        let result = self.context.query(&query)?;
        Ok(result.next().is_some())
    }

    pub fn validate_pattern(&self, pattern: &NeuralPattern) -> Result<bool, Box<dyn std::error::Error>> {
        let query = format!(
            "valid_neural_pattern({}, {}).",
                            pattern.confidence,
                            pattern.quantum_phase
        );
        let result = self.context.query(&query)?;
        Ok(result.next().is_some())
    }

    pub fn check_phase_alignment(&self, phase: f64, threshold: f64) -> Result<bool, Box<dyn std::error::Error>> {
        let query = format!("phase_aligned({}, {}).", phase, threshold);
        let result = self.context.query(&query)?;
        Ok(result.next().is_some())
    }

    pub fn verify_coherence(&self, pattern: &NeuralPattern) -> Result<bool, Box<dyn std::error::Error>> {
        let query = format!(
            "pattern_coherent({}, {}).",
                            pattern.pattern_hash,
                            pattern.coherence
        );
        let result = self.context.query(&query)?;
        Ok(result.next().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prolog_initialization() {
        let bridge = PrologBridge::new();
        assert!(bridge.init_quantum_rules().is_ok());
    }

    #[test]
    fn test_quantum_state_validation() {
        let bridge = PrologBridge::new();
        bridge.init_quantum_rules().unwrap();

        let state = BrainState::Processing;
        assert!(bridge.query_state(&state, 0.8).unwrap());
        assert!(!bridge.query_state(&state, 0.5).unwrap());
    }

    #[test]
    fn test_pattern_validation() {
        let bridge = PrologBridge::new();
        bridge.init_quantum_rules().unwrap();

        let pattern = NeuralPattern::new(0.8);
        assert!(bridge.validate_pattern(&pattern).unwrap());
    }

    #[test]
    fn test_phase_alignment() {
        let bridge = PrologBridge::new();
        bridge.init_quantum_rules().unwrap();

        assert!(bridge.check_phase_alignment(0.8, 0.75).unwrap());
        assert!(!bridge.check_phase_alignment(0.7, 0.75).unwrap());
    }
}
