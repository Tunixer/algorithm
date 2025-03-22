module;   // 告知编译器这是一个module
#include <iostream>

export module CountingIsNotFunEasy;	// 对外导出模块mymath

export int add(int a, int b) {
    return a + b;
}

export int multiply(int a, int b) {
    return a * b;
} 

export void solve() {
    std::cout << "Solving CountingIsNotFunEasy problem" << std::endl;
}

