pub mod claim;
pub mod data;
pub mod permission;
pub mod token;

pub trait ResourcePermission {
    const BIT: u32;
    const RESOURCE: &'static str;

    /// Returns all (resource, bit) pairs that must be satisfied.
    /// Default: single requirement from BIT + RESOURCE.
    /// Override for multi-permission checks.
    fn requirements() -> &'static [(&'static str, u32)] {
        &[]
    }
}

/// Combine multiple permissions into a single check.
/// All listed permissions must be satisfied.
///
/// # Example
/// ```ignore
/// combine_perms!(CanReadAndDeleteItem => [CanReadItem, CanDeleteItem]);
/// ```
#[macro_export]
macro_rules! combine_perms {
    ($name:ident => [$($perm:ty),+ $(,)?]) => {
        pub struct $name;
        impl ResourcePermission for $name {
            // Not used directly — requirements() is the source of truth
            const BIT: u32 = 0;
            const RESOURCE: &'static str = "";

            fn requirements() -> &'static [(&'static str, u32)] {
                &[
                    $((<$perm as ResourcePermission>::RESOURCE, <$perm as ResourcePermission>::BIT),)+
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! define_resource_perms {
    ($($struct_name:ident => ($bit:expr, $resource:expr)),*) => {
        $(
            pub struct $struct_name;
            impl ResourcePermission for $struct_name {
                const BIT: u32 = $bit;
                const RESOURCE: &'static str = $resource;
            }
        )*
    };
}
