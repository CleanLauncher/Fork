pub mod allocator;

#[cfg(feature = "enable_profiler")]
#[global_allocator]
static ALLOC: allocator::TrackingAllocator = allocator::TrackingAllocator;

pub fn memory_usage() -> usize {
    allocator::get_current_memory_usage()
}
