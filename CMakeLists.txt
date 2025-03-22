cmake_minimum_required(VERSION 3.28)  # 从 cmake3.28 开始支持C++20
project(AlgorithmProject CXX)

# set(CMAKE_EXPERIMENTAL_CXX_MODULE_CMAKE_API "2182bf5c-ef0d-489a-91da-49dbc3090d2a")

# Default to C++ extensions being off. Clang's modules support have trouble
# with extensions right now and it is not required for any other compiler
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_CXX_STANDARD 20)
add_library(algorithm_cxx_modules)

# target_sources()用来为目标（库或可执行文件）添加源文件。
target_sources(algorithm_cxx_modules
  PUBLIC  # 表示该模块可以被其它目标使用
    FILE_SET cxx_modules TYPE CXX_MODULES FILES  # 指定源文件的类型为C++模块
    src/CodeForce/CountingIsNotFunEasy_2063F1/CountingIsNotFunEasy.cppm
)
add_executable(algorithm src/main.cpp)
target_link_libraries(algorithm PRIVATE algorithm_cxx_modules)

