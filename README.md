# JustCode - Backend

## Setup for mobile app testing

### Requirements

* Docker installed ([link](https://docs.docker.com/get-docker/))

### Command

`docker build -t {tag} -f Dockerfile.{lang}_service . && docker run -p {port}:8080 {tag}`

Arguments:
* `{lang}` - one of supported languages:
  * `dart`
  * `flutter`
  * `full`
* `{port}` - port on which service will be exposed
* `{tag}` - tag (name) of container

## Setup for development

### Requirements

* Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Commands

* `cargo run -p {package}` - build and run app
* `cargo test` - run tests
* `cargo clippy --all-targets --all-features -- -D warnings` - analyze code with clippy

## Deployment

TODO
