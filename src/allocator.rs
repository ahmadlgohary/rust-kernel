use x86_64::{
    VirtAddr,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB
    }
};

// // Linked List Allocator using the linked list allocator crate
use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap= LockedHeap::empty();


// // Bump (Stack) Allocator using the custom bump allocator implementation
// pub mod bump;
// use bump::BumpAllocator;
// #[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> =  Locked::new(BumpAllocator::new());

// // Linked List Allocator using the custom linked list allocator implementation
// pub mod linked_list;
// use linked_list::LinkedListAllocator;
// #[global_allocator]
// static ALLOCATOR: Locked<LinkedListAllocator> =  Locked::new(LinkedListAllocator::new());

// // Fixed Block Size Allocator using the custom fixed block size allocator implementation
// pub mod fixed_size_block;
// use fixed_size_block::FixedSizeBlockAllocator;
// #[global_allocator]
// static ALLOCATOR: Locked<FixedSizeBlockAllocator> =  Locked::new(FixedSizeBlockAllocator::new());

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 K bits

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<(), MapToError<Size4KiB>> {
        let page_range = {
            let heap_start = VirtAddr::new(HEAP_START as u64);
            let heap_end = heap_start + HEAP_SIZE -1u64;
            let heap_start_page = Page::containing_address(heap_start);
            let heap_end_page = Page::containing_address(heap_end);
            Page::range_inclusive(heap_start_page, heap_end_page)
        };

        for page in page_range {
            let frame = frame_allocator
                        .allocate_frame()
                        .ok_or(MapToError::FrameAllocationFailed)?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

            unsafe {mapper.map_to(page, frame, flags, frame_allocator)?.flush()};
        }

        unsafe {ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE)};
        Ok(())
}




// wrapper around spin::Mutex to permit trait implementations
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self{
        Locked{
            inner: spin::Mutex::new(inner),
        }
    }
    pub fn lock(&self) -> spin::MutexGuard<'_, A>{
        self.inner.lock()

    }
}

fn align_up(addr: usize, align: usize) -> usize {
    /*
     * align is a power of 2 so its binary representation has 1 bit set eg (dec(8) -> bin(0000 1000))
     * align - 1 would set all the lower bit set to one eg (8-1 = dec(7) -> bin(0000 0111))
     * using NOT(align - 1) would set all the upper bits of the original align to 1s eg (not(7) -> bin(1111 1000))
     * performing an AND clears all the lower bits of (addr + align - 1)
     * we want to align upwards we increase the address by align-1, this ensures it 'rounds up' to the next align nth address    
    */
    (addr + align - 1) & !(align-1)
}