extern crate crate_fuse;
use crate_fuse::FuseMount;
use std::{thread, time};
use std::fs;

fn main(){
    
    let mut sd = FuseMount::mount(
        "proc name",
        "bucket_name",
        "mount_point",
        "ak and sk",
        "url"
    ).unwrap();
    
    let ten_millis = time::Duration::from_millis(10000);
    thread::sleep(ten_millis);

    sd.unmount().unwrap();


    match fs::read("/mnt/ss/123"){
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{:?}", e)
    };

    match fs::write("/mnt/ss/1234", b"this is a test!"){
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{:?}", e)
    };

    println!("xxx");
}