FROM scratch

COPY target/wasm32-wasi/release/wasikv.wasm /wasikv.wasm
ENTRYPOINT [ "/wasikv.wasm" ]
