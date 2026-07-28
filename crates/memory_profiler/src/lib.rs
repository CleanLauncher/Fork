pub mod core;

#[cfg(feature = "enable_profiler")]
#[global_allocator]
static ALLOC: core::TrackingAllocator = core::TrackingAllocator;

pub fn memory_usage() -> usize {
    core::get_current_memory_usage()
}
