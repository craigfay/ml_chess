# About
* This repository is being used to create a container which functions as a development environment that supports C, Rust, and Libtensorflow (for C).

# Commands
* Build container: `docker build . -t tensorflow_c_rust`
* Execute a shell inside the container: `docker run -it --rm -v $(pwd):/app -w /app tensorflow_c_rust bash`

# Commands (from inside container)
* Compile binary: `gcc c_code/main.c -ltensorflow -o target/main`
* Execute binary: `./target/main`

