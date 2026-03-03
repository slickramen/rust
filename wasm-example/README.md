# rust / typescript / wasm example
## How to build
From folder root, run:
```
cd wasm-crate
wasm-pack build --target web --out-dir ../public/wasm
```
This will compile the Rust code into Wasm code (and into executable JS code).

## How to run
From folder root, run:
```zsh
npm run dev
```
which will run the program on http://localhost:5173/