# bevy-gltf-omi-physics
Adds partial support for OMI physics glTF extensions to the bevy game engine.

[simple_scene_demo.webm](https://github.com/user-attachments/assets/8dff054e-f1a6-4a04-a3fa-f3af2465c153)

## Limitations
- Only supports the [avian](https://github.com/avianphysics/avian) physics backend.
- Currently only supports [OMI_physics_shape](https://omigroup.github.io/gltf-extensions/OMI_physics_shape/) and [OMI_physics_body](https://omigroup.github.io/gltf-extensions/OMI_physics_body/)
- Convex and trimesh collision shapes are currently unsupported.
- Collisions shapes are currently not shared between entities.
- [Trigger](https://omigroup.github.io/gltf-extensions/OMI_physics_body/README.trigger/) volumes are currently unimplemented.

## Usage
Add `GltfOmiPhysicsPlugin` to your app.
```rust
App
...
    .add_plugins(bevy_gltf_omi_physics::GltfOmiPhysicsPlugin)
...
```
## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
