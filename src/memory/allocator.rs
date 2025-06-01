use crate::free_list_node::FreeListNode;
use core::ptr::NonNull;

pub struct PageAllocator {
    head: Option<NonNull<FreeListNode>>,
    pool_start: usize,
    pool_end: usize,
    page_count: usize,
}