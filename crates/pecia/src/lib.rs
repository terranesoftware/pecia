//! # pecia
//! 
//! **Text, and only text.**
//! 
//! ## Buffers
//! 
//! ```no_run
//! use pecia::{Buffer, buffers::BufferError};
//! 
//! fn buffers() -> Result<(), BufferError> {
//!     let mut vector = Buffer::vector();
//!     
//!     vector.edit(0..0, "Hello World!".as_bytes())?;
//!     let len = vector.len();
//! 
//!     let text = vector.read(0..12)?;
//! 
//!     vector.clear();
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ## Shard
//! 
//! ```no_run
//! use pecia::{Shard, shard::{ShardError, operations::{relocate::Operation, traverse::Direction, truncate::Relation}, reference::ReferenceKind}};
//! 
//! fn shard() -> Result<(), ShardError> {
//!     let mut shard = Shard::new();
//!     
//!     let edits = Vec::new();
//!     shard.change(edits, None);
//!     let first = shard.head();
//!     
//!     shard.reference(ReferenceKind::Change, "Empty Change", first)?;
//!     
//!     shard.change(Vec::new(), Some("Second Change"));
//!     let second = shard.head();
//!     
//!     let mut deltas = Vec::new();
//!     shard.traverse(&mut deltas, &first, Direction::Older)?;
//!     
//!     shard.relocate(&[first], Operation::Copy, &second)?;
//!     
//!     shard.truncate(Relation::Ancestors, &second)?;
//!     
//!     Ok(())
//! }
//! ```

#[cfg(feature = "buffers")]
pub use pecia_buffers::Buffer;
#[cfg(feature = "buffers")]
pub mod buffers {
    pub use pecia_buffers::BufferError;
    pub use pecia_buffers::Vector;
}

#[cfg(feature = "shard")]
pub use pecia_shard::Shard;
#[cfg(feature = "shard")]
pub mod shard {
    pub use pecia_shard::{Change, Reference, ShardError};
    pub mod change {
        pub use pecia_shard::change::Edit;
    }
    pub mod operations {
        pub use pecia_shard::operations::relocate;
        pub use pecia_shard::operations::traverse;
        pub use pecia_shard::operations::truncate;
    }
    pub mod reference {
        pub use pecia_shard::reference::ReferenceKind;
    }
}