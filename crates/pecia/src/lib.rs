#[cfg(feature = "buffers")]
pub use pecia_buffers::Buffer;
#[cfg(feature = "buffers")]
pub mod buffers {
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