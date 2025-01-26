#ifndef SAFETY_BRIDGE_H
#define SAFETY_BRIDGE_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

int init_safety_bridge(void);

int check_safety(
    const uint8_t* code,
    size_t code_len,
    int safety_level,
    bool enable_optimizations,
    bool check_ownership
);

int get_safety_stats(
    size_t* enchantments_count,
    bool* wild_magic_detected
);

#ifdef __cplusplus
}
#endif

#endif // SAFETY_BRIDGE_H
