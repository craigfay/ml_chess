# Compile Rust Crate to shared library
cd /app/rust_code
cargo build

# Copy shared library
cp /app/rust_code/target/debug/libml_chess.so /usr/local/lib/libml_chess.so

# Generate header files
cbindgen --output /usr/local/include/ml_chess/c_api.h

# Reconfigure linker
ldconfig

