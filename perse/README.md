
# Perse

[![Crates.io](https://img.shields.io/crates/v/perse.svg)](https://crates.io/crates/perse) [![Docs.rs](https://docs.rs/perse/badge.svg)](https://docs.rs/perse) ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/alexwatever/perse/cicd.yml)
<br>

## About

Perse is an experimental site-builder built on top of [**Leptos**](https://github.com/leptos-rs/leptos).

This project is part of the slow code movement and is very much in development.
<br>


## Setup

#### Dependencies

[**Rust Nightly**](https://rust-lang.github.io/rustup/concepts/channels.html)  
[**cargo-leptos:**](https://crates.io/crates/cargo-leptos)
```
cargo install cargo-leptos
```

#### Installation

```

```

#### Build Site

```
cargo leptos watch
```
<br>


## Tools

Use [**SQLx CLI:**](https://crates.io/crates/sqlx-cli) to manage Migrations.  
```
cargo install sqlx-cli
```

Update your [**rust-analyzer:**](https://crates.io/crates/sqlx-cli) configuration to include the following, to enable intellisense for both SSR and CSR.  
```
"rust-analyzer.cargo.features": "all",
```
