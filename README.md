# whip-analyzer

## Summary

Small CLI application to recursively analyze the outcome of multiple CD rips performed by
[whipper](https://github.com/whipper-team/whipper).

## Disclaimer

This is a quickly written utility application. The code can not be considered well written or being
stable/tested. Furthermore this program suits only my own use case, it might not suit yours.

## Prerequisites

This CLI only works if you ripped you audio CD using `whipper` and kept the automatically
generated `*.log` files that it produced.

## Local development

Run the application

```shell
cargo run -- --path PATH_TO_ROOT_DIR
```

Run the application with hot reload using [watchexec](https://github.com/watchexec/watchexec)

```shell
watchexec -r -- "cargo run -- --path PATH_TO_ROOT_DIR"
```

Run the tests

```shell
cargo test
```
