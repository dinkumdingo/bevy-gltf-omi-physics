use avian3d::math::{Quaternion, Vector};
use avian3d::prelude::*;
use bevy::asset::LoadContext;
use bevy::gltf::GltfLoaderSettings;
use bevy::gltf::extensions::{
    ErasedGltfExtensionHandler, GltfExtensionHandler, GltfExtensionHandlers,
};
use bevy::prelude::*;
use gltf;

mod omi_types;
use omi_types::OmiPhysicsBody;
use omi_types::OmiPhysicsShape;

pub struct GltfOmiPhysicsPlugin;

impl Plugin for GltfOmiPhysicsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        bevy::tasks::block_on(async {
            app.world_mut()
                .resource_mut::<GltfExtensionHandlers>()
                .0
                .write()
                .await
                .push(Box::new(GltfExtensionHandlerOmiPhysics::default()))
        });
        #[cfg(not(target_family = "wasm"))]
        app.world_mut()
            .resource_mut::<GltfExtensionHandlers>()
            .0
            .write_blocking()
            .push(Box::new(GltfExtensionHandlerOmiPhysics::default()));
        // app.add_systems(Update, spawn_colliders);
    }
}
fn parse_shapes(gltf: &gltf::Gltf) -> Option<Vec<OmiPhysicsShape>> {
    serde_json::from_value::<Vec<OmiPhysicsShape>>(
        gltf.extensions()?
            .get("OMI_physics_shape")?
            .get("shapes")?
            .clone(),
    )
    .ok()
}

fn parse_body(gltf_node: &gltf::Node) -> Option<OmiPhysicsBody> {
    serde_json::from_value::<OmiPhysicsBody>(
        gltf_node.extensions()?.get("OMI_physics_body")?.clone(),
    )
    .ok()
}

impl From<&OmiPhysicsShape> for ColliderConstructor {
    fn from(value: &OmiPhysicsShape) -> Self {
        match value {
            OmiPhysicsShape::Box { box_ } => ColliderConstructor::Cuboid {
                x_length: box_.size[0],
                y_length: box_.size[1],
                z_length: box_.size[2],
            },
            OmiPhysicsShape::Sphere { sphere } => ColliderConstructor::Sphere {
                radius: sphere.radius,
            },
            OmiPhysicsShape::Capsule { capsule } => ColliderConstructor::Capsule {
                radius: capsule.radius,
                height: capsule.height,
            },
            OmiPhysicsShape::Cylinder { cylinder } => ColliderConstructor::Cylinder {
                radius: cylinder.radius,
                height: cylinder.height,
            },
            OmiPhysicsShape::Convex { convex: _convex } => todo!(),
            OmiPhysicsShape::Trimesh { trimesh: _trimesh } => todo!(),
        }
    }
}

#[derive(Default, Clone)]
struct GltfExtensionHandlerOmiPhysics {
    shapes: Vec<OmiPhysicsShape>,
}

impl GltfExtensionHandler for GltfExtensionHandlerOmiPhysics {
    fn on_root(
        &mut self,
        _load_context: &mut LoadContext<'_>,
        gltf: &gltf::Gltf,
        _settings: &GltfLoaderSettings,
    ) {
        let Some(omi_shapes) = parse_shapes(gltf) else {
            return;
        };
        self.shapes = omi_shapes;
    }

    fn on_gltf_node(
        &mut self,
        _load_context: &mut LoadContext<'_>,
        gltf_node: &gltf::Node,
        entity: &mut EntityWorldMut,
    ) {
        let Some(body) = parse_body(gltf_node) else {
            return;
        };
        if let Some(motion) = body.motion {
            let (rigid_body, motion_data) = match motion {
                omi_types::Motion::Static(motion_data) => (RigidBody::Static, motion_data),
                omi_types::Motion::Kinematic(motion_data) => (RigidBody::Kinematic, motion_data),
                omi_types::Motion::Dynamic(motion_data) => (RigidBody::Dynamic, motion_data),
            };
            entity.insert((
                rigid_body,
                Mass(motion_data.mass),
                LinearVelocity(Vector::from_array(motion_data.linear_velocity)),
                AngularVelocity(Vector::from_array(motion_data.angular_velocty)),
                CenterOfMass(Vector::from_array(motion_data.center_of_mass)),
                AngularInertia::from_tensor(AngularInertiaTensor::new_with_local_frame(
                    Vector::from_array(motion_data.inertia_diagonal),
                    Quaternion::from_array(motion_data.inertia_orientation),
                )),
            ));
        };

        if let Some(collider) = body.collider
            && collider.shape > -1
            && let Some(shape) = self.shapes.get(collider.shape as usize)
        {
            entity.insert(ColliderConstructor::from(shape));
            // entity.insert(OmiCollider(shape.clone()));
        };

        if let Some(_trigger) = body.trigger {
            todo!();
        };
    }

    fn dyn_clone(&self) -> Box<dyn ErasedGltfExtensionHandler> {
        Box::new((*self).clone())
    }
}
