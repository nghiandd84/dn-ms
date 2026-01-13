pub mod claim;
pub mod data;
pub mod permission;
pub mod token;

pub trait ResourcePermission {
    const BIT: u32;
    const RESOURCE: &'static str;
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
