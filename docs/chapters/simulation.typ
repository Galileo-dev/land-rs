= Simulation

== Introduction

A core peice of this paper is to develop an environment to test the G-FOLD algorithm and RL method. This environment simulates the rocket's flight and the nozzle's control. The simulation is built using the Bevy game engine and the Rapier physics engine.

This simulation environment can also be used for RL training as we can run with just the Rapier physics engine allowing for many simulations to be run in parallel. This is important as RL requires many simulations to be run to allow for enough time for the agent to learn.

== Simulation Environment

==== Bevy Game Engine
The Bevy game engine is a game engine built in Rust. it is a data-driven engine @Anderson2024bevyengine that utilises entity component systems (ECS) to manage game state. This allows for a better separation of concerns and allows features and systems to be added and removed easily.

==== Rapier Physics Engine
The Rapier physics engine is a physics engine built in Rust. It is a 2D and 3D physics engine (for this project we will use the 3D physics engine) that is built on top of the n-physics engine. It is designed to be fast and efficient and is built with the goal of being used in real-time applications.

== Simulation Components

The simulation is made up of a few key components:
- Main Rocket Simulation Module
- Camera Systems
- User Interaction Systems
- Utility & Diagnostic Systems

==== Main Rocket Simulation Module
// Located in /crates/land_sim/src/cam/ directory
// Includes modules for:
// rocket_camera.rs - Likely follows the rocket during simulation
// pan_orbit_camera.rs - Probably for user-controlled camera movement
// mod.rs - Camera system organization
// Impulse joints, rigid bodies, and other physics components

==== Camera Systems
// Located in /crates/land_sim/src/cam/ directory
// Includes modules for:
// rocket_camera.rs - Likely follows the rocket during simulation
// pan_orbit_camera.rs - Probably for user-controlled camera movement
// mod.rs - Camera system organization

==== User Interaction Systems
// Gestures: /crates/land_sim/src/gestures/ directory with gesture_system.rs
// Event Handling: /crates/land_sim/src/event_mapper/ with double_click.rs

==== Utility & Diagnostic Systems
// Error handling in /crates/land_sim/src/error.rs
// Diagnostics tools in /crates/land_sim/src/utils/diagnostics.rs

