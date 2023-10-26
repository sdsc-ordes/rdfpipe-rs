#!/usr/bin/env bash
# Compare runtime of rdfpipe vs rdfpipe-rs
# hyperfine is the only dependency (besides rdfpipe and rdfpipe-rs)
set -euo pipefail

# File path to a (large) ntriples RDF dataset
DATASET="$1"
RDFPIPE_PY="rdfpipe"
RDFPIPE_RS="./target/release/rdfpipe-rs"

# Run both implementations with different number of triples
# timings are saved in timings.csv
hyperfine \
    --warmup 1 \
    -L N 1,2,3,4,5,10,15,20,50 \
    -L FMT ttl,xml \
    --export-csv timings.csv \
    "head -n {N}000 ${DATASET} | ${RDFPIPE_PY} -i nt -o {FMT} - > /dev/null" \
    "head -n {N}000 ${DATASET} | ${RDFPIPE_RS} -i nt -o {FMT} - > /dev/null"