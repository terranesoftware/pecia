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
    pub use pecia_shard::{Change, Reference};
    pub mod change {
        pub use pecia_shard::change::Edit;
    }
    pub mod reference {
        pub use pecia_shard::reference::ReferenceKind;
    }
}