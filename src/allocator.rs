use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use x86_64::{
    VirtAddr,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB
    }
};

pub mod bump;
use bump::BumpAllocator;
#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> =  Locked::new(BumpAllocator::new());

// use linked_list_allocator::LockedHeap;
// #[global_allocator]
// static ALLOCATOR: LockedHeap= LockedHeap::empty();

pub struct Dummy;

pub const HEAP_START: usize = 0x_6767_6767_0000;
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


unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout){
        panic!("dealloc should never be called")
    }
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
    pub fn lock(&self) -> spin::MutexGuard<A>{
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