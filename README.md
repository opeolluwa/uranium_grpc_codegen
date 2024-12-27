# Uranium gRPC codegen

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Usage](#usage)

## Description

Uranium gRPC protobuf files and the generated code

## Getting Started

### Dependencies

- [Rust](https://rust-lang.org)
- [NodeJs](https://nodejs.org)
- [Protobuf](https://github.com/protocolbuffers/protobuf/releases)
- [protoc-gen-grpc-web](https://github.com/grpc/grpc-web?tab=readme-ov-file)
- [Golang proto gen]()

### Installing

To install the languages toolchain, execute

```sh
just prepare
```

4. Install

## Usage

The package is not distributed on package manager, like, crates.io, npm but can
be added as a git dependency

### Use in a Typescript project

```sh
npm i git://github.com/opeolluwa/uranium_grpc_codegen#v<TAG>
```

### Use in a Rust project

```shell
cargo add --tag v<Tag> --git https://github.com/opeolluwa/uranium_grpc_codegen --no-default-features  -F server
```
