#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(allocator_api)]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::{structures::paging::Page, VirtAddr};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use os::println;
use os::allocator;
use os::task::{Task, simple_executor::SimpleExecutor};
use os::memory::{self, BootInfoFrameAllocator};

entry_point!(kernel_main);

fn kernel_main(boot_info : &'static BootInfo) -> ! {
    println!("Hello World {}", "!");

    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    println!("Not crash!");
    os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async_number: {}", number);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    os::hlt_loop();
}
