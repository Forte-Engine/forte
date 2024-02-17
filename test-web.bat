cargo build --release --target wasm32-unknown-unknown --example gltf
wasm-bindgen --no-typescript --target web --out-dir ./pkg/ --out-name "gltf" ./target/wasm32-unknown-unknown/release/examples/gltf.wasm
npx serve .