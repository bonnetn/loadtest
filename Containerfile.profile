FROM localhost/loadtest:compile

RUN apt-get update \
    && apt-get install --no-install-recommends -y linux-perf strace \
    && rm -rf /var/lib/apt/lists/*

RUN cargo test --locked --release --lib \
    v2::tester::tests::full_tester_performance_smoke_test --no-run
