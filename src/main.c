#include <stdio.h>
#include <tensorflow/c/c_api.h>

int main() {
  printf("Tensorflow version: %s\n", TF_Version());
  return 0;
}

