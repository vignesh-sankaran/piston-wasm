fetch('target/wasm32-unknown-unknown/release/piston-wasm.wasm')
.then(response => response.arrayBuffer())
.then(bytes => WebAssembly.instantiate(bytes, {}))
.then(results => {
    results.instance.exports.game_loop();
});