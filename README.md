# GRPC Gemini

This sample project uses [tonic][1] to generate a client and structs for the
`google/cloud/aiplatform` and execute a prompt.

## Running the project

 - Clone the project with `git clone --recurse-submodules`
 - Make sure the [protobuf compiler][2] is installed on your system and available to rustc in your path.
 - Execute `cargo run`.

 [1]: https://crates.io/crates/tonic
 [2]: https://github.com/protocolbuffers/protobuf
 [3]: https://git-scm.com/book/en/v2/Git-Tools-Submodules
