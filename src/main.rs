#![feature(allocator_api)]

use linked_list_allocator::LockedHeap;

fn main() {
    let mut buf: [u8; 128] = [0; 128];
    let alloc = unsafe { LockedHeap::new(&mut buf as *mut _ as usize, buf.len()) };
    println!("Hello, world!");

    println!("With global allocator");

    let global_box = Box::new(17);
    println!("Got {:?} at {:p}", global_box, global_box);

    let mut global_vec = Vec::new();
    global_vec.push(18);
    global_vec.push(19);
    println!("Got {:?} {:p}", global_vec, &global_vec[0]);

    println!("With custom allocator");

    // let custom_box = Box::new_in(20, &alloc);
    // println!("Got {:?} at {:p}", custom_box, custom_box);

    let mut custom_vec = Vec::new_in(&alloc);
    custom_vec.push(21);
    custom_vec.push(22);
    println!("Got {:?} {:p}", custom_vec, &custom_vec[0]);
}
