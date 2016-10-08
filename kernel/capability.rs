//! Implementation of capabilities.
//!
//! Capabilities are the primitive Redox uses for privilege control. A capability is nothing but a
//! piece of data, which can only be modified by certain processes.

use std::cmp;
use std::collections::BTreeMap;

/// A capability.
///
/// In Redox, capabilities are a scheme-controlled byte sequence, and the actual semantics and
/// meaning is left to the scheme server.
///
/// # Subcapabilities
///
/// Every capability is said to have a "kind", which defines how it can be passed between processes
/// or contexts. If the kind of capability X "implies" (i.e. is stronger or equal to) the kind of
/// capability Y, then Y is said to be a subcapability of Y.
pub struct Capability {
    /// The inner data.
    data: Box<[u8]>,
    /// The capability kind.
    ///
    /// This defines its copy/send semantics.
    kind: Kind,
}

impl Capability {
    /// Is this capability inheritable (i.e. possible to pass to children)?
    pub fn is_inheritable(&self) -> bool {
        self.kind >= Kind::Inherit
    }

    /// Is this capability sendable (i.e. possible to pass to arbitrary processes)?
    pub fn is_sendable(&self) -> bool{
        self.kind >= Kind::Send
    }

    /// Set the capability data (byte sequence).
    ///
    /// You ought to be extremely careful with this. The user shouldn't be able to arbitrarily
    /// control the capability data as this means the user is able to give themself arbitrary
    /// powers.
    pub fn set_data(&mut self, data: Box<[u8]>) {
        self.data = data;
    }
}

/// A capability kind.
///
/// This defines the semantics of passing, copying, transfering, and sending capabilities across
/// contexts or processes.
#[derive(Ord, PartialOrd, Clone, Copy)]
enum Kind {
    /// A static capability.
    ///
    /// This means that you cannot pass it on to other processes. It will always stay in the
    /// process.
    Static = 0,
    /// An inheritable capability.
    ///
    /// This means that I can pass it to child processes, but not arbitrary proceses. Kind of like
    /// how I can sell heroin in Somalia to my own children, but not other people's children.
    Inherit = 1,
    /// A sendable capability.
    ///
    /// This means that I can pass the capability to any arbitrary process. Note that this can be
    /// exploited in unfortunate ways and should be used very carefully. In particular, a malicious
    /// process could givethe capability to every process on the system and thus weakening
    /// security.
    Send = 2,
}

/// A set of capabilities.
pub struct CapabilitySet {
    /// The capability sequence to kind map.
    capabilities: BTreeMap<Box<[u8]>, Kind>,
}

impl CapabilitySet {
    /// Insert a capability into the set.
    pub fn insert(&mut self, capability: Capability) -> &mut CapabilitySet {
        self.capabilities.insert(capability.data, capability.kind);

        self
    }

    /// Check if `self` is subset (or equal to) `other`.
    ///
    /// This is useful for determining if you can downgrade `other` to `self` and pass it on.
    ///
    /// If `other` contains every capability or subcapability of elements in `self`, `self` is said
    /// to be a subset of `other`.
    pub fn subset_of(&self, other: &CapabilitySet) -> bool {
        // Iterate over the map of data to kinds and searching .
        for (data, kind) in self {
            if self.contains_imp(data, *kind) {
                // The lhs didn't contain the element, hence it cannot be a subset.
                return false;
            }
        }

        // Everything matched. It's a subset!
        true
    }

    /// Does this set contains this cability or a subcapability of it?
    pub fn contains(&self, elem: &Capability) -> bool {
        self.contains_imp(&elem.data, capability.kind)
    }

    /// Internal `contains` method.
    fn contains_imp(&self, data: &[u8], kind: Kind) -> bool {
        if let Some(lhs_kind) = self.get(data) {
            // The kind of the capability must be implied.
            elem.kind <= kind
        } else {
            // The lhs did not contains the capability data in question.
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_poset() {
        assert!(Kind::Static < Kind::Inherit);
        assert!(Kind::Static < Kind::Send);
        assert!(Kind::Inherit < Kind::Send);
    }
}
