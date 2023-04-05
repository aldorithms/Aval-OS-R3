use x86_64::VirtAddr;

#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

pub fn init_heap() 
{
    let heap_start = VirtAddr::new(0x_4444_4444_0000);
    let heap_size = 1024 * 1024; // 1 MiB
    let heap_end = heap_start + heap_size;
    unsafe 
    {
        ALLOCATOR
            .lock()
            .init(heap_start.as_usize(), heap_size);
    }
}