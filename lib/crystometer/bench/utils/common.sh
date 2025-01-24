#!/bin/bash

# Colors
RESET='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'

# CPU throttling detection with more samples
check_cpu_throttling() {
    local samples=5
    local threshold=500  # MHz
    local unstable=0

    for ((i=0; i<samples; i++)); do
        local start_freq=$(cat /proc/cpuinfo | grep "cpu MHz" | head -1 | awk '{print $4}')
        sleep 0.5
        local end_freq=$(cat /proc/cpuinfo | grep "cpu MHz" | head -1 | awk '{print $4}')
        local diff=$(awk -v s="$start_freq" -v e="$end_freq" 'BEGIN {print sqrt((e-s)^2)}')

        if (( $(awk -v d="$diff" -v t="$threshold" 'BEGIN {print (d > t)}') )); then
            unstable=1
            echo "⚠️  Warning: CPU frequency unstable (${start_freq}MHz → ${end_freq}MHz)"
            break
        fi
    done

    return $unstable
}

# Statistical functions
calculate_stats() {
    local values="$1"
    echo "$values" | awk '{
        n = NF
        if (n == 0) {
            print "0 0 0"
            exit
        }

        sum = 0
        for (i = 1; i <= NF; i++) {
            sum += $i
        }
        mean = sum / n

        sumsq = 0
        for (i = 1; i <= NF; i++) {
            sumsq += ($i - mean) ^ 2
        }
        stddev = sqrt(sumsq / n)

        printf "%.9f %.9f %d\n", mean, stddev, n
    }'
}

# Generate HTML report
generate_html_report() {
    local lang=$1
    local timestamp=$2
    mkdir -p results/$lang

    cat > "results/$lang/report_${timestamp}.html" << HTML
<!DOCTYPE html>
<html>
<head>
    <title>$lang Benchmark Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .container { max-width: 1200px; margin: 0 auto; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f5f5f5; }
    </style>
</head>
<body>
    <div class="container">
        <h1>$lang Benchmark Results</h1>
        <p>Timestamp: $timestamp</p>
        <table id="resultsTable"></table>
    </div>
</body>
</html>
HTML
}
