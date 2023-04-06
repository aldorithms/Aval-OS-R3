use x86_64::VirtAddr;
use x86_64::registers::control::Cr0;

#[global_allocator]
static ALLOCATOR: 
    linked_list_allocator::LockedHeap 
        = linked_list_allocator::LockedHeap::empty();

pub fn init_heap() 
{
    unsafe fn enable_mmu() 
    {
        let mut cr0 
            = Cr0::read();

        cr0.insert(Cr0::CR0_ENABLE_PAGING);

        Cr0::write(cr0);
    }

    let heap_start 
        = VirtAddr::new(0x_4444_4444_0000);

    let heap_size 
        = 1024 * 1024; // 1 MiB

    let heap_end 
        = heap_start + heap_size;

    unsafe 
    {
        ALLOCATOR
            .lock()
            .init(heap_start.as_usize(), heap_size);
    }
}