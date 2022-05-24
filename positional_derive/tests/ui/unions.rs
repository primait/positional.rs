use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {}
