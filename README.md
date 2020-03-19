# Connect 4/TOOT and OTTO
## Prerequisites
- Rust toolchain (rustup, rustc, cargo)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Run `cargo install cargo-generate` to get cargo-generate
- [npm](https://www.npmjs.com/get-npm)

## Starting project
1. Use the following template
    ```
    cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name rust-connect-four
    ```
2. `cd rust-connect-four`
3. Run `wasm-pack build` to build the project. This should generate pkg directory
4. Run `npm init wasm-app www`
5. Go into the www directory and run `npm install` so all initial dependencies are installed
6. Add the following code into **package.json** file:
```json
"dependencies": {
    "wasm-connect-four": "file:../pkg"
}
```
7. Modify **www/index.js** change "hello-wasm-pack" to "wasm-connect-four"
8. Within **www directory**
```
npm install
npm run start
```
9. Go to localhost:8080 in any browser to see the alert message appear
