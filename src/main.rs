use kafka::run_pipeline;

// Kafka entrypoint. More description can be found in `cli.rs` docs or via running with `--help` option.
fn main() {
    env_logger::init();
    run_pipeline();
}
