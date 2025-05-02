```rust

#[derive(Component, Default, Debug, Reflect)]
#[require(Name, RocketBody)]
pub struct RocketRoot;

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
#[require(Transform, RigidBody, Collider)]
pub struct RocketBody;

...
impl RocketSettings {
    ...
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        ...
        // Body
        let body_id = commands
            .spawn((
                Name::new(name.clone()),
                RocketRoot,
                RocketBody,
                RigidBody::Dynamic,
                initial_transform,
                initial_velocity,
                Collider::cylinder(body_height / 2.0, body_radius),
                ColliderMassProperties::Mass(body_dry_mass),
                AdditionalMassProperties::Mass(body_fuel_mass),
            ))
            .id();
        ...
    }
    ...
}
```
