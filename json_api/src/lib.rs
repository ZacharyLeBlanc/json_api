pub mod resource;
pub use resource::{Resource, IdentifierObject};

// Re-export #[derive(Serialize, Deserialize)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls or data formats. They
// would need to disable default features and then explicitly re-enable std.
#[cfg(feature = "json_api_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate json_api_derive;
#[cfg(feature = "json_api_derive")]
#[doc(hidden)]
pub use json_api_derive::*;