FROM gcc:latest

# Download Rustup Installer
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add rust binaries to the path
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

FROM rust:latest

# Install cbindgen, which generates C bindings for Rust
RUN cargo install --force cbindgen

# Download Tensorflow
RUN curl https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-1.15.0.tar.gz > libtensorflow.tar.gz

# Extract Tensorflow
RUN tar -C /usr/local -xzf libtensorflow.tar.gz

# Remove zipped tar
RUN rm libtensorflow.tar.gz

# Configure the linker
RUN ldconfig

# Install Vim
RUN apt update
RUN apt install vim


