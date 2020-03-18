# About
* This repository is being used to create a container which functions as a development environment that supports C, Rust, and Libtensorflow (for C).

# Commands
* Build container: `docker build . -t tensorflow_c_rust`
* Execute a shell inside the container: `docker run -it --rm -v $(pwd):/app -w /app tensorflow_c_rust bash`

# Commands (from inside container)
* Compile Rust (from inside crate): `cargo build` 
  * This will produce the static library (archive) file: `/rust_code/target/debug'libml_chess.a`
* Create C header file from rust lib `cbindgen --output /usr/local/include/ml_chess/c_api.h <path_to_crate>`
* Compile binary: `gcc c_code/main.c rust_code/target/debug/libml_chess.a -ltensorflow -ldl -lpthread -o target/main`
* Execute binary: `./target/main`

