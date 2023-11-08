#include <iostream>
int sum() { return 1; }
int sum(int a) { return a; }
int sum(int a, int b) { return a + b; }
int main(int argc, const char *argv[]) {
  std::cout << sum(10) << std::endl;
  std::cout << sum(1, 5);
  return 0;
}
