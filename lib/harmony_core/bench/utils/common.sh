#!/bin/bash

# Colors
RESET='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'

# Warmup settings
WARMUP_ITERATIONS=100
MIN_WARMUP_TIME=2  # seconds

# Check CPU throttling
check_cpu_throttling() {
    local start_freq=$(cat /proc/cpuinfo | grep "cpu MHz" | head -1 | awk '{print $4}')
    sleep 1
    local end_freq=$(cat /proc/cpuinfo | grep "cpu MHz" | head -1 | awk '{print $4}')
    local diff=$(awk -v s="$start_freq" -v e="$end_freq" 'BEGIN {print (e-s)}')
    if (( $(awk -v d="$diff" 'BEGIN {print (d*d > 10000)}') )); then
        echo "⚠️  Warning: CPU frequency unstable (${start_freq}MHz → ${end_freq}MHz)"
        return 1
    fi
    return 0
}

# Statistical functions
calculate_stats() {
    local -n times=$1
    echo "${times[@]}" | awk '
    BEGIN {
        min = 1e10
        max = -1e10
    }
    {
        sum = 0
        for (i = 1; i <= NF; i++) {
            sum += $i
            data[i] = $i
            if ($i < min) min = $i
            if ($i > max) max = $i
        }
        n = NF
        mean = sum/n

        sumsq = 0
        for (i = 1; i <= n; i++) {
            sumsq += (data[i] - mean) ^ 2
        }
        stddev = sqrt(sumsq/n)

        printf "%.9f %.9f %d\n", mean, stddev, n
    }'
}

# HTML report generation
generate_html_report() {
    local lang=$1
    local timestamp=$2
    mkdir -p results/$lang

    cat > "results/$lang/report_${timestamp}.html" << HTML
<!DOCTYPE html>
<html>
<head>
    <title>$lang Benchmark Results</title>
    <script src="https://cdn.plot.ly/plotly-latest.min.js"></script>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .container { max-width: 1200px; margin: 0 auto; }
        .chart { margin: 20px 0; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f5f5f5; }
    </style>
</head>
<body>
    <div class="container">
        <h1>$lang Benchmark Results</h1>
        <p>Timestamp: $timestamp</p>
        <div id="timeChart" class="chart"></div>
        <div id="distributionChart" class="chart"></div>
        <table id="resultsTable"></table>
    </div>
</body>
</html>
HTML
}
