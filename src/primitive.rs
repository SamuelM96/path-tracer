// use crate::utils::transform_swaps_handedness;
// use ultraviolet::Mat4;
//
// #[allow(dead_code)]
// pub struct Primitive {
//     pub object_to_world: Mat4,
//     pub world_to_object: Mat4,
//     pub reverse_orientation: bool,
//     pub transform_swaps_handedness: bool,
// }
//
// #[allow(dead_code)]
// impl Primitive {
//     pub fn new(
//         object_to_world: Mat4,
//         world_to_object: Mat4,
//         reverse_orientation: bool,
//     ) -> Primitive {
//         let transform_swaps_handedness = transform_swaps_handedness(&object_to_world);
//
//         Primitive {
//             object_to_world,
//             world_to_object,
//             reverse_orientation,
//             transform_swaps_handedness,
//         }
//     }
// }
