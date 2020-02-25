FROM gcc:latest

# Download Tensorflow
RUN curl https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-1.15.0.tar.gz > libtensorflow.tar.gz

# Extract Tensorflow
RUN tar -C /usr/local -xzf libtensorflow.tar.gz

# Remove zipped tar
RUN rm libtensorflow.tar.gz

# Configure the linker
RUN ldconfig

