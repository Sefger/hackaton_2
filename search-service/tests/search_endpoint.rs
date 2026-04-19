// Full end-to-end with real sparse model is covered by docker-compose smoke test (Phase 6).
// Fine-grained pipeline behavior is covered by unit tests in src/pipeline.rs.
#[tokio::test]
async fn compile_check() {
    // No-op test — serves as a compile boundary check.
}
