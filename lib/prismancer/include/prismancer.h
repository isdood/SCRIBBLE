/**
 * @file prismancer.h
 * @brief Main FFI interface for the Prismancer Engine
 * @version 0.1.0
 * @date 2025-01-21 18:15:29
 *
 * @copyright Copyright (c) 2025 isdood
 *
 * Primary interface header for Prismancer Engine, integrating all FFI bridges
 * and providing the main API surface for engine consumers.
 */

#ifndef PRISMANCER_H
#define PRISMANCER_H

#include <stdint.h>
#include <stdbool.h>

// Include specific bridges
#include "ffi/julia_bridge.h"
#include "ffi/zig_bridge.h"

#ifdef __cplusplus
extern "C" {
    #endif

    /* --- Version Information --- */

    #define PRISMANCER_VERSION_MAJOR 0
    #define PRISMANCER_VERSION_MINOR 1
    #define PRISMANCER_VERSION_PATCH 0
    #define PRISMANCER_VERSION_STRING "0.1.0"

    /* --- Engine Configuration --- */

    /**
     * @brief Engine configuration settings
     */
    typedef struct PrismancerConfig {
        // Core settings
        uint32_t thread_count;          // Number of worker threads
        size_t memory_pool_size;        // Size of main memory pool
        bool enable_debug_logging;       // Enable debug logs

        // Crystal settings
        double base_coherence;          // Base crystal coherence [0.0, 1.0]
        double reality_anchor;          // Reality anchoring strength
        uint32_t quantum_depth;         // Quantum simulation depth

        // Rendering settings
        uint32_t max_batch_size;        // Maximum geometry batch size
        bool enable_vulkan;             // Use Vulkan renderer
        uint32_t max_draw_calls;        // Maximum draw calls per frame

        // Physics settings
        double physics_timestep;        // Fixed physics timestep
        uint32_t physics_iterations;    // Physics iterations per frame
        bool enable_quantum_effects;    // Enable quantum physics

        // Cache settings
        size_t cache_size;             // Size of geometry cache
        double cache_coherence;        // Minimum cache coherence
        uint32_t cache_generations;    // Number of cache generations
    } PrismancerConfig;

    /**
     * @brief Engine instance handle
     */
    typedef struct PrismancerEngine {
        void* handle;                  // Opaque engine handle
        PrismancerConfig config;      // Engine configuration
        bool is_initialized;          // Initialization state
    } PrismancerEngine;

    /* --- Engine Lifecycle --- */

    /**
     * @brief Create a new engine instance with default configuration
     * @return PrismancerEngine* New engine instance or NULL on failure
     */
    PrismancerEngine* prismancer_create(void);

    /**
     * @brief Create an engine instance with custom configuration
     * @param config Custom configuration
     * @return PrismancerEngine* New engine instance or NULL on failure
     */
    PrismancerEngine* prismancer_create_with_config(const PrismancerConfig* config);

    /**
     * @brief Initialize the engine
     * @param engine Engine instance
     * @return int 0 on success, error code otherwise
     */
    int prismancer_initialize(PrismancerEngine* engine);

    /**
     * @brief Shutdown the engine and free resources
     * @param engine Engine instance
     */
    void prismancer_shutdown(PrismancerEngine* engine);

    /* --- Scene Management --- */

    /**
     * @brief Scene handle
     */
    typedef struct PrismancerScene {
        void* handle;                 // Opaque scene handle
        uint32_t entity_count;       // Number of entities
        bool is_active;             // Whether scene is active
    } PrismancerScene;

    /**
     * @brief Create a new scene
     * @param engine Engine instance
     * @return PrismancerScene* New scene or NULL on failure
     */
    PrismancerScene* prismancer_scene_create(PrismancerEngine* engine);

    /**
     * @brief Destroy a scene
     * @param scene Scene to destroy
     */
    void prismancer_scene_destroy(PrismancerScene* scene);

    /* --- Entity Management --- */

    /**
     * @brief Entity handle
     */
    typedef struct PrismancerEntity {
        uint64_t id;                 // Entity ID
        PrismancerScene* scene;     // Owner scene
        void* components;           // Component data
        bool is_active;            // Entity state
    } PrismancerEntity;

    /**
     * @brief Create a new entity in scene
     * @param scene Target scene
     * @return PrismancerEntity* New entity or NULL on failure
     */
    PrismancerEntity* prismancer_entity_create(PrismancerScene* scene);

    /**
     * @brief Destroy an entity
     * @param entity Entity to destroy
     */
    void prismancer_entity_destroy(PrismancerEntity* entity);

    /* --- Frame Management --- */

    /**
     * @brief Begin a new frame
     * @param engine Engine instance
     * @return int 0 on success, error code otherwise
     */
    int prismancer_frame_begin(PrismancerEngine* engine);

    /**
     * @brief End current frame and present
     * @param engine Engine instance
     * @return int 0 on success, error code otherwise
     */
    int prismancer_frame_end(PrismancerEngine* engine);

    /* --- Error Handling --- */

    /**
     * @brief Get last error message
     * @return const char* Error message or NULL if no error
     */
    const char* prismancer_get_last_error(void);

    /**
     * @brief Clear error state
     */
    void prismancer_clear_error(void);

    /* --- Debug Interface --- */

    /**
     * @brief Set debug log callback
     * @param callback Function pointer to debug callback
     */
    typedef void (*PrismancerDebugCallback)(const char* message);
    void prismancer_set_debug_callback(PrismancerDebugCallback callback);

    /**
     * @brief Get engine statistics
     * @param engine Engine instance
     * @param stats Output statistics structure
     * @return int 0 on success, error code otherwise
     */
    typedef struct PrismancerStats {
        double frame_time;          // Last frame time
        uint32_t draw_calls;       // Draw calls this frame
        size_t memory_used;        // Current memory usage
        double cache_hit_rate;     // Cache hit rate
        uint32_t entity_count;     // Total entities
        double coherence;          // Average coherence
    } PrismancerStats;

    int prismancer_get_stats(const PrismancerEngine* engine, PrismancerStats* stats);

    /* --- Utility Functions --- */

    /**
     * @brief Convert error code to string
     * @param error_code Error code to convert
     * @return const char* Error string
     */
    const char* prismancer_error_string(int error_code);

    /**
     * @brief Get engine version string
     * @return const char* Version string
     */
    const char* prismancer_version(void);

    /**
     * @brief Check if current system is compatible
     * @return bool true if compatible, false otherwise
     */
    bool prismancer_check_compatibility(void);

    #ifdef __cplusplus
}
#endif

#endif /* PRISMANCER_H */
