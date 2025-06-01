pub struct FreeListNode {
    next: Option<Box<FreeListNode>>,
}