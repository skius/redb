mod base;
mod bitmap;
mod buddy_allocator;
mod cached_file;
pub mod file_backend;
mod header;
mod in_memory_backend;
mod layout;
mod lru_cache;
mod page_manager;
mod region;
mod savepoint;
#[allow(clippy::pedantic, dead_code)]
mod xxh3;

pub(crate) use base::{Page, PageHint, PageNumber, MAX_PAIR_LENGTH, MAX_VALUE_LENGTH};
pub(crate) use header::PAGE_SIZE;
pub use in_memory_backend::InMemoryBackend;
pub(crate) use page_manager::{xxh3_checksum, TransactionalMemory, FILE_FORMAT_VERSION2};
pub use savepoint::Savepoint;
pub(crate) use savepoint::SerializedSavepoint;

pub(super) use base::{PageImpl, PageMut};
pub(crate) use buddy_allocator::BuddyAllocator;
pub(super) use xxh3::hash128_with_seed;
