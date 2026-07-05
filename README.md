# RailLatch

RailLatch is a Rust parser and replay library for offline railway interlocking
event recorder archives. It models the kind of bundle produced by yard edge
controllers when signal, switch, and track-circuit equipment must keep running
while disconnected from a central dispatch system. A single archive can contain
a symbol dictionary, track topology snapshot, event sessions, relay journals,
maintenance scripts, and opaque controller blobs gathered from the field.

The library decodes the outer archive table, validates section bounds and
checksums, reconstructs track nodes and edges, replays route and switch events,
executes a compact maintenance-script bytecode, and produces an analysis report
that summarizes safety findings. The code is intentionally stateful: later
stages depend on records decoded earlier, so fuzzing exercises multi-stage
parser behavior instead of a flat byte decoder.

The repository includes cargo-fuzz style fuzz targets, a recognized seed corpus
layout, and a ClusterFuzzLite build script that builds all harnesses with only
local path dependencies. The harnesses cover archive decoding, stream decoding,
script execution, topology scoring, and relay journal replay.
