macro_rules! show {
    (??) => {
        println!("{} = {}",stringify!($field),??);
    };
}
//
// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }
// struct Dimensions {
//     width: f32,
//     height: f32,
// }
//
// impl Point {
//     fn new(x: i32, y: i32, z: i32) -> Self {
//         Point {
//             x,
//             y,
//             z,
//         }
//     }
// }
macro_rules! make_struct_with_new {
    (
        $(struct $struct_name:ident {
            $($field:ident: $type:ty),* $(,)?
        }),*
    ) => {
        $(
            struct $struct_name {
                $($field: $type),*
            }

            impl $struct_name {
                pub fn new($($field: $type),*) -> Self {
                    Self {
                        $($field),*
                    }
                }
            }
        )*
    };
}
make_struct_with_new!(
    struct Point{
        x:i32,
        y:i32,
        z:i32
    },

    struct Dimensions {
        width: f32,
        height: f32,
    }
);
#[allow(dead_code)]
pub fn main() {}


