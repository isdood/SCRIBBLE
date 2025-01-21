#!/bin/bash

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Running Lazuline benchmarks..."

# Run benchmarks
cargo bench

# Generate benchmark report
echo -e "\nGenerating benchmark report..."
REPORT_DIR="target/criterion/report"
mkdir -p "$REPORT_DIR"

echo "<!DOCTYPE html>
<html>
<head>
    <title>Lazuline Benchmark Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2em; }
        .benchmark { margin-bottom: 2em; }
        .chart { margin: 1em 0; }
    </style>
</head>
<body>
    <h1>Lazuline Benchmark Results</h1>
    <p>Generated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')</p>
    <div class='benchmark'>
        <h2>Results</h2>
        <pre>" > "$REPORT_DIR/index.html"

# Append benchmark results
cargo bench --no-run --message-format=json \
    | jq -r 'select(.profile.test == true) | .filenames[]' \
    | while read -r benchmark; do
        "$benchmark" --bench | tee -a "$REPORT_DIR/index.html"
    done

echo "</pre>
    </div>
</body>
</html>" >> "$REPORT_DIR/index.html"

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ Benchmarks completed!"
echo "View the report at target/criterion/report/index.html"
