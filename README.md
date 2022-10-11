# Redis SIMD JSON

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![npm version](https://img.shields.io/npm/v/redis-simd-json)](https://www.npmjs.com/package/redis-simd-json)
[![Cargo](https://img.shields.io/crates/v/redis-simd-json.svg)](https://crates.io/crates/redis-simd-json)
[![Documentation](https://docs.rs/redis-simd-json/badge.svg)](https://docs.rs/redis-simd-json)


Blazingly fast N-API bindings for native Redis client GET/SET/MGET/MSET commands utilizing SIMD JSON serialization with connection multiplexing via [redis-swapplex](https://crates.io/crates/redis-swapplex).

Additionally, this library adds support for compare and swap if [redis-cas](https://github.com/Bajix/redis-cas/) is installed on the connected Redis server.

The native Redis client can be configured using ENV variables (see [env-url](https://crates.io/crates/env-url) for usage):

```
REDIS_URL=redis://127.0.0.1:6379
# Override env mapping for easy kubernetes config
REDIS_HOST_ENV=MONOLITH_STAGE_REDIS_MASTER_PORT_6379_TCP_ADDR
REDIS_PORT_ENV=MONOLITH_STAGE_REDIS_MASTER_SERVICE_PORT_REDIS
```

Use `RUST_TARGET` env variable to set compilation target when cross-compiling:
```
RUST_TARGET=aarch64-unknown-linux-gnu
```

Other targets include:
```
x86_64-apple-darwin
x86_64-pc-windows-msvc
x86_64-unknown-linux-musl
```