
# Perse

[![Crates.io](https://img.shields.io/crates/v/perse.svg)](https://crates.io/crates/perse) [![Docs.rs](https://docs.rs/perse/badge.svg)](https://docs.rs/perse) ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/alexwatever/perse/cicd.yml)
<br>


## About

Perse is an experimental site-builder built on top of [**Leptos**](https://github.com/leptos-rs/leptos).

This project is part of the slow code movement and is very much in development.
<br>


## Roadmap

- [x] Database integration with SQLx
- [x] Basic routing functionality
- [x] Create new view functionality
- [x] Homepage functionality
- [ ] Edit view functionality
- [ ] Delete view functionality
- [ ] Improve UI/UX design
- [ ] Enhance error handling and logging
- [ ] Dataframes project
- [ ] Rebuilder project
- [ ] Auth
- [ ] Add support for plugins and extensions
- [ ] Integration with third-party service
- [ ] Improve performance and scalability
<br>


## Setup

#### Dependencies

[**Rust Nightly**](https://rust-lang.github.io/rustup/concepts/channels.html)  
[**cargo-leptos:**](https://crates.io/crates/cargo-leptos)
```
cargo install cargo-leptos
```
<br>


## Tools

Use [**SQLx CLI:**](https://crates.io/crates/sqlx-cli) to manage Migrations.  
```
cargo install sqlx-cli
```

Update your VS Code [**rust-analyzer:**](https://crates.io/crates/sqlx-cli) configuration to include the following, to enable intellisense for both SSR and CSR.  
```
"rust-analyzer.cargo.features": "all",
```
