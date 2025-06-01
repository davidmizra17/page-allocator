use crate::free_list_node::FreeListNode;
pub struct Page {
    page_frame_number: u64,
    start_address: usize,
}