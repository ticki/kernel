//! Resource struct

/// A resource identifier.
// TODO: Close on exec
#[derive(Copy, Clone, Debug)]
pub struct Resource {
    /// The scheme that this resource refers to
    pub scheme: usize,
    /// The number the scheme uses to refer to this resource
    pub number: usize,
}
