#include <stdio.h>
#include <tensorflow/c/c_api.h>
#include <ml_chess/c_api.h>

int main() {
  printf("Tensorflow version: %s\n", TF_Version());
  printf("%d\n", sum(5,6));
  return 0;
}

