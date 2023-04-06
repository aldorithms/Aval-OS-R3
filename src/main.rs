#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

mod gdt;
mod idt;
mod interrupts;
mod memory;
mod panic;
mod serial;
mod Process;
mod Scheduler;

use alloc::boxed::Box;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR:
     linked_list_allocator::LockedHeap 
        = linked_list_allocator::LockedHeap::empty();

#[no_mangle]
pub extern "C" fn _start()
    -> ! 
    {
        serial_println!("Kernel initialized");
        gdt::init();
        idt::init();
        unsafe 
        { 
            interrupts::PICS
                .lock()
                .initialize() 
        };
        x86_64::instructions::interrupts::enable();

        memory::init_heap();

        // Initialize the timer
        init_timer(TIMER_INTERVAL_US);
    
        // Create some sample processes
        let mut process1 
            = create_process()
                .unwrap();
        let mut process2 
            = create_process()
                .unwrap();
    
        // Set the starting state of the processes
        process

        let _allocated_box 
            = Box::new(42);

        serial_println!("Box allocated at {:p}", _allocated_box);
        loop 
        {
            round_robin_scheduler();
        }
    }

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) 
    -> ! 
    {
        panic!("allocation error: {:?}", layout)
    }