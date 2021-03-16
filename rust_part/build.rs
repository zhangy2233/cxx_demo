
use std::{convert::TryInto, env, path::PathBuf};
use std::process::Command;
use cxx_build;


use std::fs;
use std::path::Path;

fn main() {

    println!("cargo:warning=Error failed with {:?}.", "start bin");

    // Compile cpp
    let base_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=CARGO_MANIFEST_DIR: {:?}", base_dir);

    // Compile handle1 
    let cmd = format!("cd {:?} && 
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
    cp ./handle2.cpp {:?}/include/handle2.cpp", base_dir, base_dir, base_dir, base_dir, base_dir).to_string();
    let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .expect("出错");
    println!("cargo:warning=stdout: {:?}", String::from_utf8(output.stdout));

    // start
    println!("cargo:rustc-link-search=all=include");
    println!("cargo:rustc-link-lib=handle1");

    

    cxx_build::bridge("src/main.rs")
    .file("include/handle2.cpp")
    .include("include")
    .flag_if_supported("-std=c++11")
    .compile("cxx-demo");

    
}