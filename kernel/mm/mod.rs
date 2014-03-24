pub use self::allocator::{
	Allocator,
	BuddyAlloc,
	Alloc,
};

pub mod allocator;
pub mod physical;
pub mod virtual;
