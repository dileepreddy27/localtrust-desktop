# Verification record

Local machine: Windows, Node.js 24.15.0. No Rust or C++ toolchain was available locally at implementation time.

- Browser adapter tests and JavaScript syntax checks are run locally.
- Native Rust/C++ tests, lint, and desktop compilation are verified through the repository's CI workflow; consult the exact commit's workflow status.
- No live Ollama/model inference, quantization, native GUI interaction, installer, or macOS build has been measured.
- Browser demo uses in-memory synthetic data. It does not exercise native filesystem enforcement.
- No latency, throughput, or security benchmark claims are made.
