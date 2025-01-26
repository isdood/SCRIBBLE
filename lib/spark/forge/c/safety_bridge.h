#ifndef CRYSTAL_SAFETY_BRIDGE_H
#define CRYSTAL_SAFETY_BRIDGE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

int init_safety_bridge(void);
int check_safety(const uint8_t* code, size_t code_len, int safety_level,
                bool enable_optimizations, bool check_ownership);

#endif
