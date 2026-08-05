

# Perse

[![Crates.io](https://img.shields.io/crates/v/perse.svg)](https://crates.io/crates/perse) [![Docs.rs](https://docs.rs/perse/badge.svg)](https://docs.rs/perse) ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/alexwatever/perse/cicd.yml)
<br>


## Acerca de

Perse es un constructor de sitios web experimental construido sobre [**Leptos**](https://github.com/leptos-rs/leptos).

Este proyecto forma parte del movimiento de código lento y está en plena etapa de desarrollo.
<br>


## Hoja de ruta

- [x] Integración de base de datos con SQLx
- [x] Funcionalidad básica de enrutamiento
- [x] Funcionalidad para crear nuevas vistas
- [x] Funcionalidad de página de inicio
- [ ] Funcionalidad para editar vistas
- [ ] Funcionalidad para eliminar vistas
- [ ] Mejorar el diseño de UI/UX
- [ ] Mejorar el manejo de errores y registros
- [ ] Proyecto Dataframes
- [ ] Proyecto Rebuilder
- [ ] Autenticación
- [ ] Añadir soporte para plugins y extensiones
- [ ] Integración con servicios de terceros
- [ ] Mejorar el rendimiento y la escalabilidad
<br>


## Configuración

#### Dependencias

[**Rust Nightly**](https://rust-lang.github.io/rustup/concepts/channels.html)  
[**cargo-leptos:**](https://crates.io/crates/cargo-leptos)
```
cargo install cargo-leptos
```
<br>


## Herramientas

Usa [**SQLx CLI:**](https://crates.io/crates/sqlx-cli) para gestionar las migraciones.  
```
cargo install sqlx-cli
```

Actualiza la configuración de [**rust-analyzer:**](https://crates.io/crates/sqlx-cli) en tu VS Code para incluir lo siguiente, y habilitar intellisense tanto para SSR como para CSR.  
```
"rust-analyzer.cargo.features": "all",
```
