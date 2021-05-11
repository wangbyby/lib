
# rust+js+wasm

## 1. 安装工具

```bash
# 1. install wasm-pack

# 2.  
cargo install cargo-generate

# 3. install npm
npm install npm@latest -g

# 4. 为rust安装wasm32-unknown-unknown
rustup target install wasm32-unknown-unknown

```

## 2. build

```bash
# 1. wasm build
wasm-pack build

# 2. npm build
npm install
npm run start
```