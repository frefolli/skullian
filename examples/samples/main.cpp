#include <iostream>

void bar() {
    std::cout << 2 << std::endl;
    if (1 == 2) {
      foo();
    }
}

void foo() {
    std::cout << 1 << std::endl;
    bar();
}

int main() {
    foo();
    return 0;
}
