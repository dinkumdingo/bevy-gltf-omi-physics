use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Box {
    pub size: [f32; 3],
}

impl Default for Box {
    fn default() -> Self {
        Box {
            size: [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Sphere {
    pub radius: f32,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere { radius: 0.5 }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Capsule {
    pub radius: f32,
    pub height: f32,
}

impl Default for Capsule {
    fn default() -> Self {
        Capsule {
            radius: 0.5,
            height: 2.0,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            radius: 0.5,
            height: 2.0,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Convex {
    pub mesh: isize,
}

impl Default for Convex {
    fn default() -> Self {
        Convex { mesh: -1 }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Trimesh {
    pub mesh: isize,
}

impl Default for Trimesh {
    fn default() -> Self {
        Trimesh { mesh: -1 }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OmiPhysicsShape {
    Box {
        #[serde(rename = "box")]
        box_: Box,
    },
    Sphere {
        sphere: Sphere,
    },
    Capsule {
        capsule: Capsule,
    },
    Cylinder {
        cylinder: Cylinder,
    },
    Convex {
        convex: Convex,
    },
    Trimesh {
        trimesh: Trimesh,
    },
}

// impl OmiPhysicsShape {
//     pub fn as_shared_shape(&self) -> Option<SharedShape> {
//         match self {
//             OmiPhysicsShape::Box { box_ } => Some(SharedShape::cuboid(
//                 box_.size[0] / 2.0,
//                 box_.size[1] / 2.0,
//                 box_.size[2] / 2.0,
//             )),
//             OmiPhysicsShape::Sphere { sphere } => Some(SharedShape::ball(sphere.radius)),
//             OmiPhysicsShape::Capsule { capsule } => {
//                 Some(SharedShape::capsule_y(capsule.height, capsule.radius))
//             }
//             OmiPhysicsShape::Cylinder { cylinder } => Some(SharedShape::cylinder(
//                 cylinder.height / 2.0,
//                 cylinder.radius,
//             )),
//             OmiPhysicsShape::Convex { convex: _ } => {
//                 todo!()
//             }
//             OmiPhysicsShape::Trimesh { trimesh: _ } => {
//                 todo!()
//             }
//         }
//     }
// }
//
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OmiPhysicsBody {
    pub motion: Option<Motion>,
    pub collider: Option<Collider>,
    pub trigger: Option<Trigger>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Motion {
    Static(MotionData),
    Kinematic(MotionData),
    Dynamic(MotionData),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct MotionData {
    pub mass: f32,
    pub linear_velocity: [f32; 3],
    pub angular_velocty: [f32; 3],
    pub center_of_mass: [f32; 3],
    pub inertia_diagonal: [f32; 3],
    pub inertia_orientation: [f32; 4],
}

impl Default for MotionData {
    fn default() -> Self {
        Self {
            mass: 1.0,
            linear_velocity: [0.0, 0.0, 0.0],
            angular_velocty: [0.0, 0.0, 0.0],
            center_of_mass: [0.0, 0.0, 0.0],
            inertia_diagonal: [0.0, 0.0, 0.0],
            inertia_orientation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Collider {
    pub shape: isize,
}

impl Default for Collider {
    fn default() -> Self {
        Self { shape: -1 }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Trigger {
    pub shape: isize,
    pub nodes: Vec<isize>,
}

impl Default for Trigger {
    fn default() -> Self {
        Self {
            shape: -1,
            nodes: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let expected_box = OmiPhysicsShape::Box {
            box_: Box {
                size: [5.12212, 1.1, 6.0],
            },
        };
        let test_string = r#"{ "type": "box", "box": { "size": [5.12212, 1.1, 6] } }"#;
        let test_box: OmiPhysicsShape = serde_json::from_str(test_string).unwrap();
        assert_eq!(test_box, expected_box);
    }

    #[test]
    #[should_panic]
    fn test_box_invalid_size() {
        let expected_box = OmiPhysicsShape::Box {
            box_: Box::default(),
        };
        let test_string = r#"{ "type": "box", "box": { "size": [1, 2] } }"#;
        let test_box: OmiPhysicsShape = serde_json::from_str(test_string).unwrap();
        assert_eq!(test_box, expected_box);
    }

    #[test]
    fn test_box_default() {
        let expected_box = OmiPhysicsShape::Box {
            box_: Box::default(),
        };
        let test_string = r#"{ "type": "box", "box": {} }"#;
        let test_box: OmiPhysicsShape = serde_json::from_str(test_string).unwrap();
        assert_eq!(test_box, expected_box);
    }

    #[test]
    fn test_sphere() {
        let expected = OmiPhysicsShape::Sphere {
            sphere: Sphere { radius: 0.1 },
        };
        let test_string = r#"{ "type": "sphere", "sphere": { "radius": 0.1 } }"#;
        let test: OmiPhysicsShape = serde_json::from_str(test_string).unwrap();
        assert_eq!(test, expected);
    }

    #[test]
    fn test_cylinder_default() {
        let expected = OmiPhysicsShape::Cylinder {
            cylinder: Cylinder::default(),
        };
        let test_string = r#"{ "type": "cylinder", "cylinder": { "radius": 0.5, "height": 2.0 } }"#;
        let test: OmiPhysicsShape = serde_json::from_str(test_string).unwrap();
        assert_eq!(test, expected);
    }

    #[test]
    #[should_panic]
    fn invalid_shape() {
        let test_string = r#"{}"#;
        let _: OmiPhysicsShape = serde_json::from_str(test_string).unwrap();
    }

    #[test]
    fn test_body() {
        let expected = OmiPhysicsBody {
            motion: Some(Motion::Static(MotionData::default())),
            collider: Some(Collider { shape: 0 }),
            trigger: None,
        };
        let test_string = r#"{"motion":{"type":"static"},"collider":{"shape":0}}"#;
        let test: OmiPhysicsBody = serde_json::from_str(test_string).unwrap();
        assert_eq!(test, expected);
    }
}
