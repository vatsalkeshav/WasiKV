# WasiKV – A Tiny WebAssembly Key-Value Store

This is a demo project that shows:

- A simple in-memory key-value store written in Rust
- Compiled to WebAssembly
- Runs on Kubernetes using Runwasi or Wasmtime
- Uses GitHub Actions to build and deploy to KinD

## Endpoints

- `/put?foo=bar` — Store a key-value pair
- `/get?key=foo` — Retrieve a value

## Quickstart

```bash
cargo build --target wasm32-wasi --release
docker build -t ghcr.io/vatsalkeshav/wasikv .
kubectl apply -f k8s/
```
