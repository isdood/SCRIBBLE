/**
 * @file julia_bridge.h
 * @brief FFI Bridge between Julia and Prismancer Engine
 * @version 0.1.0
 * @date 2025-01-21
 *
 * @copyright Copyright (c) 2025 isdood
 *
 * This header defines the interface between Prismancer's core systems
 * and Julia-based physics/quantum calculations.
 */

#ifndef PRISMANCER_JULIA_BRIDGE_H
#define PRISMANCER_JULIA_BRIDGE_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
    #endif

    /* --- Type Definitions --- */

    /**
     * @brief Quantum state representation for crystal-based physics
     */
    typedef struct PrismancerQuantumState {
        double coherence;          // Quantum coherence value [0.0, 1.0]
        double phase;             // Phase angle in radians
        double amplitude;         // Wave function amplitude
        uint32_t energy_level;    // Discrete energy state
        bool is_collapsed;        // Whether the state has been observed
    } PrismancerQuantumState;

    /**
     * @brief Wave function parameters for physics calculations
     */
    typedef struct PrismancerWaveFunction {
        double* coefficients;     // Array of wave function coefficients
        size_t coeff_count;      // Number of coefficients
        double frequency;        // Oscillation frequency
        double decay_rate;       // Quantum decay rate
    } PrismancerWaveFunction;

    /**
     * @brief Physical properties of a crystal entity
     */
    typedef struct PrismancerCrystalPhysics {
        double mass;             // Mass in simulation units
        double charge;           // Electrical charge
        double spin;            // Quantum spin value
        double* position;       // 3D position vector
        double* velocity;       // 3D velocity vector
        double* forces;         // Applied forces vector
    } PrismancerCrystalPhysics;

    /* --- Initialization and Cleanup --- */

    /**
     * @brief Initialize the Julia runtime and physics systems
     * @return int 0 on success, error code otherwise
     */
    int prismancer_julia_init(void);

    /**
     * @brief Cleanup Julia runtime and release resources
     */
    void prismancer_julia_cleanup(void);

    /* --- Quantum State Management --- */

    /**
     * @brief Create a new quantum state
     * @param coherence Initial coherence value
     * @return PrismancerQuantumState* Pointer to new state or NULL on failure
     */
    PrismancerQuantumState* prismancer_quantum_state_create(double coherence);

    /**
     * @brief Destroy a quantum state and free resources
     * @param state State to destroy
     */
    void prismancer_quantum_state_destroy(PrismancerQuantumState* state);

    /**
     * @brief Evolve quantum state according to Schrödinger equation
     * @param state State to evolve
     * @param dt Time step
     * @return int 0 on success, error code otherwise
     */
    int prismancer_quantum_state_evolve(PrismancerQuantumState* state, double dt);

    /* --- Wave Function Operations --- */

    /**
     * @brief Create a new wave function
     * @param size Number of coefficients
     * @param frequency Initial frequency
     * @return PrismancerWaveFunction* Pointer to new wave function or NULL on failure
     */
    PrismancerWaveFunction* prismancer_wave_function_create(size_t size, double frequency);

    /**
     * @brief Destroy a wave function and free resources
     * @param wf Wave function to destroy
     */
    void prismancer_wave_function_destroy(PrismancerWaveFunction* wf);

    /**
     * @brief Compute wave function interference between two states
     * @param wf1 First wave function
     * @param wf2 Second wave function
     * @param result Output interference pattern
     * @return int 0 on success, error code otherwise
     */
    int prismancer_wave_interference_compute(
        const PrismancerWaveFunction* wf1,
        const PrismancerWaveFunction* wf2,
        PrismancerWaveFunction* result
    );

    /* --- Physics Simulation --- */

    /**
     * @brief Create a new crystal physics object
     * @param mass Initial mass
     * @return PrismancerCrystalPhysics* Pointer to new physics object or NULL on failure
     */
    PrismancerCrystalPhysics* prismancer_crystal_physics_create(double mass);

    /**
     * @brief Destroy a crystal physics object and free resources
     * @param physics Physics object to destroy
     */
    void prismancer_crystal_physics_destroy(PrismancerCrystalPhysics* physics);

    /**
     * @brief Update physics simulation for a single step
     * @param physics Physics object to update
     * @param dt Time step
     * @return int 0 on success, error code otherwise
     */
    int prismancer_physics_step(PrismancerCrystalPhysics* physics, double dt);

    /**
     * @brief Apply quantum forces to crystal physics object
     * @param physics Physics object
     * @param state Quantum state affecting the object
     * @return int 0 on success, error code otherwise
     */
    int prismancer_apply_quantum_forces(
        PrismancerCrystalPhysics* physics,
        const PrismancerQuantumState* state
    );

    /* --- Error Handling --- */

    /**
     * @brief Get the last error message from Julia runtime
     * @return const char* Error message or NULL if no error
     */
    const char* prismancer_julia_get_last_error(void);

    /**
     * @brief Clear the last error state
     */
    void prismancer_julia_clear_error(void);

    /* --- Version Information --- */

    /**
     * @brief Get the Julia bridge version string
     * @return const char* Version string
     */
    const char* prismancer_julia_version(void);

    /**
     * @brief Check if Julia version is compatible
     * @return bool true if compatible, false otherwise
     */
    bool prismancer_julia_check_compatibility(void);

    #ifdef __cplusplus
}
#endif

#endif /* PRISMANCER_JULIA_BRIDGE_H */
