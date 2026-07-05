# RailLatch

RailLatch is a Rust parser and replay library for offline railway interlocking
event recorder archives. A single archive can contain a symbol dictionary,
track topology, event sessions, relay journals, maintenance scripts, and opaque
controller blobs gathered from a disconnected yard.

The repository includes cargo-fuzz style fuzz targets, a recognized seed corpus
layout, and a ClusterFuzzLite build script that builds all harnesses with only
local path dependencies.
