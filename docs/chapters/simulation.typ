= Simulation

== Introduction

A core piece of this paper is to develop an environment to test the @gfold algorithm and RL method. This environment simulates the rocket's flight and the nozzle's control. The simulation is built using the Bevy game engine and the Rapier physics engine.

This simulation environment can also be used for RL training as we can run with run the Bevy and Rapier physics engine in headless mode allowing for many simulations to be run in parallel by not rendering to the screen. This is important as RL requires many simulations to be run to allow enough time for the agent to learn.

== Simulation Environment

=== Bevy Game Engine
The Bevy game engine is a game engine built in Rust. it is a data-driven engine that utilises entity component systems (ECS) to manage game state. This allows for a better separation of concerns and allows many features and systems to be added while keeping the codebase maintainable. It also allows for easy extensibility and modularity @Anderson2024bevyengine.

==== Bevy ECS
Bevy @ecs is a critical part of the Bevy game engine. It is a powerful framework that allows use to seperate data (Components), logic (Systems), and entities (Entity IDs) for efficent game logic implementation. Some of the the key features of Bevy @ecs are:
- *Entity*: A unique identifier for an object in the game world (e.g. a player, a rocket, a landing pad).
- *Component*: A piece of data attached to entities, defining their properties (e.g. a rocket's position and velocity, rocket rigidbody).
- *System*: This is the logic that actually manipulates the entities with the specific components (e.g. a system that updates the rocket engine thrust depending on the keyboard input).

#include "../code/ecs.typ"

==== Rapier Physics Engine
The Rapier physics engine is built in Rust. It supports both 2D and 3D physics however we will use the 3D physics engine for this project. Most relevant to this project Rapier supports joint constraints, collision detection, and rigid body dynamics. This is important as we need to simulate the rocket's flight trajectory, engine thrust, and nozzle control @Crozet2024dimforge.


== Simulation Components

The simulation is made up of a few key components:
- Main Rocket Simulation Module
- Camera Systems
- User Interaction Systems
- Utility & Diagnostic Systems

==== Main Rocket Simulation Module
#include "../code/rocket.typ"
