cmake_minimum_required(VERSION 3.14.1)
SET(CMAKE_BUILD_TYPE "Release")
project(handle2)
set(CMAKE_CXX_STANDARD 14)

# add_executable(handle2 handle2.cpp)

add_library(handle2 SHARED handle2.h handle2.cpp) 

string(REGEX REPLACE "(.*)/(.*)/(.*)" "\\1" PROJECT_INIT_PATH  ${PROJECT_SOURCE_DIR})
# message("上层目录=" ${PROJECT_INIT_PATH})

target_link_libraries(handle2 ${PROJECT_INIT_PATH}/cxx_demo/handle1/libhandle1.a)













