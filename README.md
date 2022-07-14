# juno_faucet_frontend

### What is this?

A front end for the Junox faucet hosted [here](https://faucet.roguenet.io/)

Could also be used as boilerplate for wasm web apps

Powered by [Juno Network](https://twitter.com/junonetwork)

#### Front end stack (Rust)

 - [Yew](https://docs.rs/yew/0.19.3/yew/) - Framework for building wasm web apps
 - [Trunk](https://crates.io/crates/trunk) - Wasm web app bundler 
 - [Wasm-bindgen](https://docs.rs/wasm-bindgen/0.2.81/wasm_bindgen/) - Library for wasm <> javascript interactions
 - [Gloo net](https://docs.rs/gloo-net/0.2.3/gloo_net/) - Library of HTTP wrappers for wasm apps
 - [Bech32](https://docs.rs/bech32/latest/bech32/index.html) - Library for encoding/decoding bech 32 format
---

### Why?

1) We saw need for an open junox faucet API in the ecosystem
2) We put the faucet on a roguenet.io page to help spread awareness about our main project
3) Builders gonna build
---

### Usage


- You could use the app to get some testnet juno
- You could fork this front end to use as boilerplate for a wasm web application (just rip out anything you don't need)
- You could make your own front end & connect it to our faucet API (via basic HTTP posts)
---


### Wanna fork?


Requirements

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) - Programming language for 🦀
- [Trunk](https://trunkrs.dev/) - Wasm web app bundler
---

Set target
`$ rustup target add wasm32-unknown-unknown`

Clone this repo: `
$ git clone https://github.com/LeTurt333/juno_faucet`

In the root of the repo you just cloned, run  
`$ trunk serve`

Open a browser window & go to 
`http://127.0.0.1:8080/`