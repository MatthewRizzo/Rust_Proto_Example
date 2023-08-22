# Test Proto Depends

## Why Make this Example

Protobufs are very useful for auto-generating easily serializable /
de-serializable data.
As protobufs can be used between languages, there are a few diffent project layouts employed.
The most common are as follow, often times the shared `.proto`'s are stored:

1. In a shared repository
   1. Where all consumer's (regardless of their language) can consume that shared `.proto`.
   2. Example repo name:
      1. FakeService
2. In a isolated repository
   1. All consumers are in the same repository together, but different that the `.proto`'s
   2. Example repos:
      1. FakeService-Proto
      2. FakseService
3. One repository for the `.proto`'s, and one repository each language-based consumer.
   1. Example repos:
      1. FakeService-Proto
      2. FakeService-Cpp
      3. FakeService-rs
      4. FakeService-js

Rust in particular has an idiomatic way to handle auto-generated code: using
`build.rs` build scripts and calling the corresponding generator.
In the proto use case listed above, most likely, the shared `.proto` repository
should be responsible for generating the bindings and exporting their library
This paradigm can make it tricky in all common use cases (except for the first).

This repository, with its example, serves as a concrete suggestion for how
to approach the problem.
Especially when the `.proto`'s are not in the same repository as the consumer.
Much of it can be applied to projects in general.

Essentially, all that is being done is the auto-generation of Rust-bindings
for the protobuf's described by the `.proto`'s, and exporting them under the

```Rust
// crate::<proto_file_name>::<proto name>
// In this case,
crate::export::GenericRequest;
```

Hence, consumers of this library can access them via:

```Rust
use proto_generator::protos;
use proto_generator::protos::GenericRequest;
```

## How to Use

Once your setup is completed, the `proto_generator` library can be consumed by
another library, or directly by a binary.

### Consuming Generated Proto-Rust in Library

[Lib Cargo.toml]: ./example_lib_consumer/Cargo.toml
[Lib Consumption]: ./example_lib_consumer/src/proto_consumer.rs

#### Adding Generated Bindings as Dependency in Library Cargo.toml

Follow the example in [Lib Cargo.toml] of adding the generator
library as a dependency. In this case, a path was used, however, it could also
be a github url. For example:

```bash
proto_generator = { git = "ssh://git@github.com:<username>/<repo_name>.git", branch = "main" }
```

#### Consuming Generated Bindings in Library Program

Now that the generator library is added as a dependency to our library, we can
`use` the proto binding inside of our library as in [Lib Consumption].

### Consuming Generated Proto-Rust in Binary

[Bin Cargo.toml]: ./example_bin_consumer/Cargo.toml
[Bin Consumption]: ./example_bin_consumer/src/proto_consumer.rs

#### Adding Generated Bindings as Dependency in Binary Cargo.toml

Similar to with the library, the [Bin Cargo.toml] adds `proto_generator` as a
dependency.

#### Consuming Generated Bindings in Binary Program

As the generator has the bindings as a dependency, you can include them, as
done below:

```Rust
use proto_generator::export::GenericRequest;
```

A larger example can be seen in [Bin Consumption].

## Extensions and Changes Required

[proto_generator build]: ./proto_generator/build.rs
[proto_generator lib]: ./proto_generator/lib.rs

If applying this template to your project, the following changes are suggested:

* Adapt the top level [[proto_generator build]] script to include the desired `.proto` files
  * Change / add more `.inputs()`'s with the path to the `.proto` files
    * This will result in more `.proto` -> `.rs` files being created.
    * Make sure to use the `root_dir` pattern already employed.
* For every new `.proto` file added to inputs, make sure to have them wrapped.
  * Just copy the `wrap_output_rust_proto()` with the path to the generated binding
  * The pattern is `generated_protos/<proto file name>.rs`
* Include all new bindings in the [proto_generator lib].
  * The path should mimic the wrapped script used in the [proto_generator build]
  * I.e. `generated_protos/<proto file name>.rs`

## Setup Requirements

[Protobuf Docs]: https://grpc.io/docs/protoc-installation/#install-pre-compiled-binaries-any-os

The only setup needed is to have protoc installed and added to path.
See [Protobuf Docs] for more details.

```bash
PB_REL="https://github.com/protocolbuffers/protobuf/releases"
curl -LO $PB_REL/download/v3.15.8/protoc-2.28.8-linux-x86_64.zip
unzip protoc-3.15.8-linux-x86_64.zip -d $HOME/.local

# Add to Path in your .bashrc
export PATH="$PATH:$HOME/.local/bin"
```

## Running the Example

```bash
cargo run
```
