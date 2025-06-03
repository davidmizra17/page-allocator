use core::ptr::NonNull;
pub struct FreeListNode {
    pub next: Option<NonNull<FreeListNode>>,
}