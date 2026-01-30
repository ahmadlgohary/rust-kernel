use super::Locked;
use core::{mem, ptr};
use alloc::alloc::{GlobalAlloc, Layout};

struct ListNode {
    next: Option<&'static mut ListNode>
}

// block sizes start from 8 bytes, since each block would need to store a 64-bit pointer to a ListNode
// for allocations greater than 2048 bytes we will fall back to the linked list allocator
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        FixedSizeBlockAllocator { 
            list_heads: [EMPTY; BLOCK_SIZES.len()], 
            fallback_allocator: linked_list_allocator::Heap::empty() 
        }
    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize){
        self.fallback_allocator.init(heap_start, heap_size);
    }

    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut()
        }
    }

    fn list_index(layout: &Layout) -> Option<usize> {
        let require_block_size = layout.size().max(layout.align());
        BLOCK_SIZES.iter().position(|&s| s>= require_block_size)
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let mut allocator = self.lock();
        match FixedSizeBlockAllocator::list_index(&layout) {
            Some(index) => {
                // memory to be allocated fits in the fixed sized blocks
                
                match allocator.list_heads[index].take(){
                    Some(node) => {
                        // head of list exists, so we pop and return it 
                        // then make the next node the new head
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        // list is empty for the current fixed block size
                        // get the block size and use the same block alignment as the block size
                        // (only works for sizes of power of 2)
                        // create a new layout with size `block_size` and alignment `block_align` 
                        // which is equal to block size
                        // use the fallback allocator (LinkedList) to perform the allocation
                        // The block will be added to the `list_heads` during deallocation
                        let block_size = BLOCK_SIZES[index];
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();
                        allocator.fallback_alloc(layout)
                    }
                }
            }
            // memory to be allocated is larger than the largest fixed sized block
            // use linked list allocator as a fallback
            None => allocator.fallback_alloc(layout)
        }
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout){
        let mut allocator = self.lock();
        match FixedSizeBlockAllocator::list_index(&layout) {
            Some(index) => {
                // deallocated memory's size ∈ BLOCK_SIZES

                // create a new node and point it to the head of the list
                // this sets the head of the list to None too
                // head of list could be None or could be a ListNode
                let new_node = ListNode{next: allocator.list_heads[index].take()};

                // this is in case the memory size is smaller than the size of the ListNode struct, 
                // which shouldn't happen in our case
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                // convert the *mut u8 pointer to a as *mut ListNode
                let new_node_ptr = ptr as *mut ListNode;

                // write the ListNode to memory 
                new_node_ptr.write(new_node);

                // head of the list is now None so we set the new node as the new head of the list 
                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                // deallocated memory is larger than the largest fixed size block
                // use fallback allocator (LinkedList) to deallocate it

                // The fallback allocator's deallocate() expects a ptr of type NonNull 
                // so we convert the pointer from *mut u8 to NonNull
                let ptr = ptr::NonNull::new(ptr).unwrap(); 
                allocator.fallback_allocator.deallocate(ptr, layout);
            }
        }

    }
}