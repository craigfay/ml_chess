FROM gcc:latest

# Download Tensorflow
RUN curl https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-1.15.0.tar.gz > /usr/lib/libtensorflow.tar.gz


RUN curl https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-1.15.0.tar.gz > libtensorflow-cpu-linux-x86_64-1.15.0.tar.gz 

# Extract Tensorflow
RUN tar -C /usr/local -xzf libtensorflow-cpu-linux-x86_64-1.15.0.tar.gz 

# Configure the linker
RUN ldconfig

