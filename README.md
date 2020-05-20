# About
* This repository is being used to create a container which functions as a development environment that supports C, Rust, and Libtensorflow (for C).

# Commands
* Build container: `docker build . -t ml_chess`
* Execute a shell inside the container: `docker run -it --rm -v $(pwd):/app -w /app ml_chess bash`

# test
