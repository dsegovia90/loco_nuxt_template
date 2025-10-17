# Loco Nuxt Template

[live demo](https://loco-nuxt-template.dsegovia.io/)

[Loco](https://loco.rs) API framework running on Rust, nuxt as a front end.




## Quick Start

```sh
cargo loco start

# in another terminal
cd frontend && bun run dev
```

## Key Template Features

### Frontend with Nuxt
Statically built. Uses bun but you can use any other bundler.

The nuxt dev server is proxied to loco thanks to vite easy proxy config.

[Pinia](https://pinia.vuejs.org/) as a state management library.
[nuxtui](https://ui.nuxt.com/) to build the UI (uses tailwind under the hood), replace as necessary.
[zod](https://zod.dev/) for schema validation (great pair with ts-rs)

### ts-rs Bindings
Uses [ts-rs](https://docs.rs/ts-rs/latest/ts_rs/) to generate bindings between Rust and TypeScript. This helps tremendously with development speed and code quality. Keep in mind that you need to use some tweaks when using sea-orm Decimal `#[ts(type = "number")]`, see [here](https://docs.rs/ts-rs/latest/ts_rs/trait.TS.html#struct-field-attributes), as well as other types.

To generate bindings, run:
```sh
cargo test
```

### Parallel Tests
Rust tests run in parallel. I understand this is quite opinionated, but I've worked on large projects where parallel tests were just a no brainer. Specially when stress testing. Also, fearless concurrency.

You can still run tests in serial using the [serial_test](https://docs.rs/serial_test/latest/serial_test/) crate as loco intended.

To achieve this, the tests only truncate data once per `cargo test` run. See app.rs -> truncate function for more details. This also means that tests' db data can collide with each other, keep that in mind when writing tests. When all else fails, you can always run tests in serial.
