// use cgmath::prelude::*;
// use cgmath::Matrix4;
// use uuid::Uuid;

// pub struct Transform {
//     uuid: Uuid,
//     name: String,
//     matrix: Matrix4<f32>,
// }

// impl Node for Transform {
//     fn new(uuid: Uuid, name: String) -> Self {
//         Transform {
//             uuid,
//             matrix: Matrix4::identity(),
//             name,
//         }
//     }
//     fn uuid(&self) -> Uuid {
//         self.uuid.clone()
//     }
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//     fn set_name(&mut self, name: String) {
//         self.name = name;
//     }
// }
