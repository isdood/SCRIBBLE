#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <libgen.h>
#include <limits.h>
#include <errno.h>

int main(int argc, char *argv[]) {
    char path[PATH_MAX];
    char exe_path[PATH_MAX];
    char *dir;
    ssize_t len;

    // Get directory from environment or executable path
    dir = getenv("SPARK_SEED_DIR");
    if (!dir) {
        len = readlink("/proc/self/exe", exe_path, sizeof(exe_path) - 1);
        if (len == -1) {
            perror("Failed to get executable path");
            return 1;
        }
        exe_path[len] = '\0';
        dir = dirname(exe_path);
    }

    // Build path to real binary
    if (snprintf(path, sizeof(path), "%s/zig-out/bin/seed", dir) >= sizeof(path)) {
        fprintf(stderr, "Path too long\n");
        return 1;
    }

    // Execute the real binary with raw arguments
    execv(path, argv);
    
    // If execv fails, print error and exit
    perror("Failed to execute seed manager");
    return 1;
}
