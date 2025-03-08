===== 1. Define components
```rust
use bevy::prelude::*;

// Define components
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
```

===== 2. Spawn Entities
```rust
fn setup(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}
```

===== 3. Create Systems
Systems define queries to explicitly define which entities and components they need to read from or write to during the game loop.
Queries are specified in the function parameters and specify component access (immutable or mutable). We can also filter and impose entity inclusion/exclusion rules.
```rust
fn movement_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x * time.delta_seconds();
        position.y += velocity.y * time.delta_seconds();
    }
}
```
In these examples:
- `Query<(&mut Position, &Velocity)>` specifies that the system must have both a `Position` and `Velocity` component.
  - `Position` is mutable as it is being updated.
  - `Velocity` is immutable we only need to read from it.

Bevy makes sure that systems safely borrow components in adherence to Rust's borrowing rules, preventing data races and ensuring memory safety.

===== 4. Register with App

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement_system)
        .run();
}
```


