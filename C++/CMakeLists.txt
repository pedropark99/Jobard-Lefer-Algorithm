
cmake_minimum_required(VERSION 3.22)

project(even_spaced_curves CXX)
include_directories(PUBLIC ./../include/)
link_libraries(m)
add_executable(even_spaced_curves src/main.cpp)

