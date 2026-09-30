# Verification record

Local machine: Windows, Node.js 24.15.0. No Rust or C++ toolchain was available locally at implementation time.

- Two browser adapter tests passed locally, using `node --test --test-isolation=none tests/demo.test.mjs` because this sandbox prevents the test runner from spawning subprocesses. Standard `npm test` passed in both CI operating systems.
- JavaScript syntax checks passed. The headless browser test passed deny, approved create, read-back, traversal rejection, mobile overflow, and no uncaught browser errors. `docs/demo.png` is the inspected synthetic screenshot.
- Initial native core CI passed 8 Rust tests on Linux and 7 on Windows (the symlink test is Unix-only), plus Clippy with warnings denied.
- Linux native Tauri desktop compilation passed in [workflow 36649844516](https://github.com/dileepreddy27/localtrust-desktop/actions/runs/36649844516), alongside both core jobs. Its resolved desktop dependency lockfile is committed. This is compilation evidence, not native GUI execution evidence.
- Native Rust/C++ tests, lint, and desktop compilation are verified through the repository's CI workflow; consult the exact commit's workflow status.
- No live Ollama/model inference, quantization, native GUI interaction, installer, or macOS build has been measured.
- Browser demo uses in-memory synthetic data. It does not exercise native filesystem enforcement.
- No latency, throughput, or security benchmark claims are made.
