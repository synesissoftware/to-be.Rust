// truthy.rs

/// Trait that provides truthy attributes for an implementing type.
pub trait Truthy {
    /// Indicates whether the instance can be classed as "falsey".
    fn is_falsey(&self) -> bool {
        Some(false) == self.is_truthy()
    }
    /// Indicates whether the instance can be classed as "truey".
    fn is_truey(&self) -> bool {
        Some(true) == self.is_truthy()
    }
    /// Indicates whether the instance can be classed as "truthy", and, if
    /// so, whether it is "truey" or "falsey".
    fn is_truthy(&self) -> Option<bool>;
}


// ///////////////////////////// end of file //////////////////////////// //
