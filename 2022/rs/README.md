# Rust solutions 2022

(done long after the fact :P )

## Development

### Common commands

All commands assume a working directory of a dayXX unless otherwise specified.

#### Generate a new day directory

In the rs directory:

```shell
$ cargo generate --path daily-template
```

#### Run a test, rerun on code changes

```shell
$ cargo watch -x check -x 'test part1'
```

#### Run a test, rerun on code changes, show prints

```shell
$ cargo watch -x check -x 'test part1 -- --nocapture'
```

#### Run solution

```shell
$ cargo run --bin part1
```

