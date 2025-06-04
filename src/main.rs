mod memory;
use memory::allocator::PageAllocator;
fn main() {
       // Simulate a memory pool as a vector of bytes
       const PAGE_SIZE: usize = 4096;
       const PAGE_COUNT: usize = 10;
       let mut pool = vec![0u8; PAGE_SIZE * PAGE_COUNT];
   
       // Get the start and end addresses of the pool
       let test = pool.as_mut_ptr();
       let pool_start = pool.as_mut_ptr() as usize;
       println!("this is the pool pointer or whatevvvv {:?}", test);
       let pool_end = pool_start + pool.len();
   
       // Initialize the allocator
       let mut allocator = PageAllocator::new();
       allocator.init(pool_start, pool_end, PAGE_SIZE);
   
       // Try to allocate 5 pages
       for i in 0..5 {
           let page = allocator.allocate();
           match page {
               Some(ptr) => {
               
                println!("Allocated page {} at address: {:p}", i, ptr);
               let page_addr = ptr.as_ptr() as *mut u8;
               unsafe { *page_addr = (i + 1) as u8; }
               }
               None => println!("Failed to allocate page {}", i),
           }
       }

      // Now, print the first byte of each allocated page by calculating the offset
for i in 0..5 {
    let page = allocator.allocate();
    if let Some(ptr) = page {
        let page_addr = ptr.as_ptr() as usize;
        let offset = page_addr - pool.as_ptr() as usize;
        println!(
            "First byte of allocated page {}: {}",
            i,
            pool[offset]
        );
    }
}
}
