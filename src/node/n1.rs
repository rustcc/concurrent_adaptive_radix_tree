#[repr(C)]
#[repr(align(64))]
pub struct N1 {
    header: NodeHeader,
    value: V,
}