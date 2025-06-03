use super::free_list_node::FreeListNode;
use core::ptr::NonNull;

pub struct PageAllocator {
    head: Option<NonNull<FreeListNode>>,
    pool_start: usize,
    pool_end: usize,
    page_count: usize,
}

impl PageAllocator {
    pub fn new() -> Self {
        Self { 
            head: None,
            pool_start: 0,
            pool_end: 0,
            page_count: 0,
        }
    }

    pub fn init(&mut self, pool_start: usize, pool_end: usize, page_size: usize) {
        assert!(pool_start < pool_end);
        assert_eq!(pool_start % page_size, 0);
        assert_eq!(pool_end % page_size, 0);

        self.pool_start = pool_start;
        self.pool_end = pool_end;
        self.page_count = (pool_end - pool_start) / page_size;

        let mut current = pool_start;
        let mut prev_node: Option<NonNull<FreeListNode>> = None;

        while current + page_size <= pool_end {
            let node_ptr = current as *mut FreeListNode;

            unsafe {
                let node = node_ptr.as_mut().unwrap();
                node.next = prev_node;

                prev_node = Some(NonNull::new_unchecked(node_ptr));
            }
            current+= page_size;
        }
        self.head = prev_node;
    }

    pub fn allocate(&mut self) -> Option<NonNull<FreeListNode>>{
        //check if list is empty
        if self.head.is_none(){
            return None;
        }

        //pop head of the link list
        let node = self.head.unwrap();
        let next = unsafe{ node.as_ref().next };
        self.head = next;

        return Some(node);

    }
}