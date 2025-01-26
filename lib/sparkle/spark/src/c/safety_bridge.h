#ifndef SAFETY_BRIDGE_H
#define SAFETY_BRIDGE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Safety level definitions
#define SAFETY_LEVEL_CALM     0
#define SAFETY_LEVEL_BALANCED 1
#define SAFETY_LEVEL_WILD     2

int init_safety_bridge(void);

int check_safety(
    const uint8_t* code,
    size_t code_len,
    int safety_level,
    bool enable_optimizations,
    bool check_ownership
);

int get_safety_stats(void);

#ifdef __cplusplus
}
#endif

#endif // SAFETY_BRIDGE_H
