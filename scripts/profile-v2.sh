#!/bin/sh
set -eu

image="localhost/loadtest:profile"
test_name="v2::tester::tests::full_tester_performance_smoke_test"
profile_dir="$(pwd)/profiles"

mkdir -p "$profile_dir"

podman build -f Containerfile.profile -t "$image" .

case "${1:-perf-stat}" in
    perf-stat)
        exec podman run --privileged --rm "$image" \
            perf stat -d \
            cargo test --locked --release --lib "$test_name" \
            -- --ignored --nocapture
        ;;
    perf-record)
        exec podman run --privileged --rm \
            -v "$profile_dir:/profiles:Z" "$image" \
            perf record -g --call-graph dwarf \
            -o /profiles/perf.data \
            cargo test --locked --release --lib "$test_name" \
            -- --ignored --nocapture
        ;;
    strace)
        exec podman run --privileged --rm "$image" \
            strace -f -c \
            cargo test --locked --release --lib "$test_name" \
            -- --ignored --nocapture
        ;;
    *)
        echo "usage: $0 [perf-stat|perf-record|strace]" >&2
        exit 2
        ;;
esac
