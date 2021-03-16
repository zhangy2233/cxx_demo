

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("include/handle2.h");
        fn handle2(v:i32) ->i32;
    }
}


fn main(){
    ffi::handle2(123);
}