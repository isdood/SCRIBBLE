/**
 * @file zig_bridge.h
 * @brief FFI Bridge between Zig and Prismancer Engine
 * @version 0.1.0
 * @date 2025-01-21 18:12:47
 *
 * @copyright Copyright (c) 2025 isdood
 *
 * This header defines the interface between Prismancer's core systems
 * and Zig-based low-level performance components.
 */

#ifndef PRISMANCER_ZIG_BRIDGE_H
#define PRISMANCER_ZIG_BRIDGE_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
    #endif

    /* --- Type Definitions --- */

    /**
     * @brief Memory pool for efficient resource management
     */
    typedef struct PrismancerMemoryPool {
        void* handle;           // Opaque handle to Zig memory pool
        size_t total_size;      // Total size of the pool
        size_t used_size;       // Currently used size
        uint32_t block_size;    // Size of individual blocks
        bool is_locked;         // Thread safety lock
    } PrismancerMemoryPool;

    /**
     * @brief Geometry batch for efficient rendering
     */
    typedef struct PrismancerGeometryBatch {
        uint32_t vertex_count;   // Number of vertices
        uint32_t index_count;    // Number of indices
        void* vertex_data;       // Raw vertex data
        void* index_data;        // Raw index data
        uint32_t material_id;    // Associated material
        bool is_dynamic;         // Whether data can change
    } PrismancerGeometryBatch;

    /**
     * @brief Cache entry for crystal state
     */
    typedef struct PrismancerCacheEntry {
        uint64_t key;           // Cache key
        void* data;            // Cached data
        size_t size;           // Size of data
        double coherence;      // Crystal coherence value
        uint64_t timestamp;    // Last access time
        bool is_volatile;      // Whether entry can be evicted
    } PrismancerCacheEntry;

    /**
     * @brief Vulkan resource handle wrapper
     */
    typedef struct PrismancerVulkanHandle {
        uint64_t handle;        // Raw Vulkan handle
        uint32_t type;         // Resource type
        bool is_owned;         // Whether we own this resource
    } PrismancerVulkanHandle;

    /* --- Memory Management --- */

    /**
     * @brief Create a new memory pool
     * @param total_size Total size in bytes
     * @param block_size Size of individual blocks
     * @return PrismancerMemoryPool* New pool or NULL on failure
     */
    PrismancerMemoryPool* prismancer_memory_pool_create(size_t total_size, uint32_t block_size);

    /**
     * @brief Destroy a memory pool and free all resources
     * @param pool Pool to destroy
     */
    void prismancer_memory_pool_destroy(PrismancerMemoryPool* pool);

    /**
     * @brief Allocate memory from pool
     * @param pool Target pool
     * @param size Requested size
     * @return void* Allocated memory or NULL on failure
     */
    void* prismancer_memory_allocate(PrismancerMemoryPool* pool, size_t size);

    /**
     * @brief Free memory back to pool
     * @param pool Target pool
     * @param ptr Memory to free
     */
    void prismancer_memory_free(PrismancerMemoryPool* pool, void* ptr);

    /* --- Geometry Processing --- */

    /**
     * @brief Create a new geometry batch
     * @param vertex_count Initial vertex capacity
     * @param index_count Initial index capacity
     * @return PrismancerGeometryBatch* New batch or NULL on failure
     */
    PrismancerGeometryBatch* prismancer_geometry_batch_create(
        uint32_t vertex_count,
        uint32_t index_count
    );

    /**
     * @brief Destroy a geometry batch
     * @param batch Batch to destroy
     */
    void prismancer_geometry_batch_destroy(PrismancerGeometryBatch* batch);

    /**
     * @brief Process geometry for rendering
     * @param batch Batch to process
     * @param coherence Crystal coherence value
     * @return int 0 on success, error code otherwise
     */
    int prismancer_geometry_process(PrismancerGeometryBatch* batch, double coherence);

    /* --- Cache Management --- */

    /**
     * @brief Create a new cache entry
     * @param key Cache key
     * @param data Initial data
     * @param size Size of data
     * @return PrismancerCacheEntry* New entry or NULL on failure
     */
    PrismancerCacheEntry* prismancer_cache_entry_create(
        uint64_t key,
        const void* data,
        size_t size
    );

    /**
     * @brief Destroy a cache entry
     * @param entry Entry to destroy
     */
    void prismancer_cache_entry_destroy(PrismancerCacheEntry* entry);

    /**
     * @brief Update cache coherence
     * @param entry Entry to update
     * @param coherence New coherence value
     * @return int 0 on success, error code otherwise
     */
    int prismancer_cache_update_coherence(PrismancerCacheEntry* entry, double coherence);

    /* --- Vulkan Integration --- */

    /**
     * @brief Create a new Vulkan handle wrapper
     * @param raw_handle Raw Vulkan handle
     * @param type Resource type
     * @return PrismancerVulkanHandle* New handle or NULL on failure
     */
    PrismancerVulkanHandle* prismancer_vulkan_handle_create(uint64_t raw_handle, uint32_t type);

    /**
     * @brief Destroy a Vulkan handle wrapper
     * @param handle Handle to destroy
     */
    void prismancer_vulkan_handle_destroy(PrismancerVulkanHandle* handle);

    /**
     * @brief Submit geometry batch to Vulkan
     * @param handle Vulkan handle
     * @param batch Geometry batch
     * @return int 0 on success, error code otherwise
     */
    int prismancer_vulkan_submit_geometry(
        PrismancerVulkanHandle* handle,
        const PrismancerGeometryBatch* batch
    );

    /* --- SIMD Operations --- */

    /**
     * @brief Check SIMD support level
     * @return uint32_t Bitmask of supported SIMD features
     */
    uint32_t prismancer_simd_get_support(void);

    /**
     * @brief Process vertex data using SIMD
     * @param data Vertex data
     * @param count Vertex count
     * @return int 0 on success, error code otherwise
     */
    int prismancer_simd_process_vertices(void* data, size_t count);

    /* --- Error Handling --- */

    /**
     * @brief Get the last error message from Zig runtime
     * @return const char* Error message or NULL if no error
     */
    const char* prismancer_zig_get_last_error(void);

    /**
     * @brief Clear the last error state
     */
    void prismancer_zig_clear_error(void);

    /* --- Version Information --- */

    /**
     * @brief Get the Zig bridge version string
     * @return const char* Version string
     */
    const char* prismancer_zig_version(void);

    /**
     * @brief Check if Zig version is compatible
     * @return bool true if compatible, false otherwise
     */
    bool prismancer_zig_check_compatibility(void);

    #ifdef __cplusplus
}
#endif

#endif /* PRISMANCER_ZIG_BRIDGE_H */
