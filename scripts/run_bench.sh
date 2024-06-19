#!/usr/bin/env bash
# Compare runtime of two rdfpipe-rs binaries
# hyperfine is the only dependency
set -euo pipefail

# File path to a (large) ntriples RDF dataset
RDFPIPE_BIN1="$1"
RDFPIPE_BIN2="$2"
DATASET="$3"

# Run both implementations with different number of triples
# timings are saved in timings.csv
hyperfine \
    --warmup 1 \
    -L N 1,2,3,4,5,10,15,20,50 \
    -L FMT ttl,xml \
    --export-csv timings.csv \
    "head -n {N}000 ${DATASET} | ${RDFPIPE_BIN1} -i nt -o {FMT} - > /dev/null" \
    "head -n {N}000 ${DATASET} | ${RDFPIPE_BIN2} -i nt -o {FMT} - > /dev/null"
