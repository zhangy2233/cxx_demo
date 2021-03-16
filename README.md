## I want to achieve the following functions: Use a module of C++ in Rust, we will name this module handle2 for now, handle2 needs some functions in a dynamic link library (handle1). In the process of using ffi::handle2(123), the unavoidable "undefined reference to'handle1'" error has always appeared. What should I do?


## How can we use "handle2" in rust via "CXX" if we only have files in the include folder and we don't have the source code for "handle1"


## What you do here is compile "handle1" and copy some files into the include folder, emulating the environment
``` shell
cd {:?} && 
    cd ../handle1 && 
    cmake . && make  && 
    rm -r CMakeFiles && 
    rm cmake_install.cmake && 
    rm CMakeCache.txt && 
    rm Makefile &&
    cp ./handle1.h {:?}/include/handle1.h &&
    cp ./libhandle1.so {:?}/include/libhandle1.so &&
    cd ../handle2 &&
    cp ./handle2.h {:?}/include/handle2.h &&
    cp ./handle2.cpp {:?}/include/handle2.cpp
```



``` shell
├── handle1
│   ├── CMakeLists.txt
│   ├── handle1.cpp
│   ├── handle1.h
│   └── libhandle1.so
├── handle2
│   ├── CMakeLists.txt
│   ├── handle2.cpp
│   └── handle2.h
├── README.en.md
├── README.md
└── rust_part
    ├── build.rs
    ├── Cargo.toml
    ├── include
    │   ├── handle1.h
    │   ├── handle2.cpp
    │   ├── handle2.h
    │   └── libhandle1.so
    └── src
        └── main.rs
```