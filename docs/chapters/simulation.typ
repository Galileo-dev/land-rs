#import "@preview/codly:1.2.0": *
#import "@preview/codly-languages:0.1.1": *

= Simulation

== Introduction

A core piece of this paper is to develop an environment to test the @gfold algorithm and RL method. This environment simulates the rocket's flight and the nozzle's control. The simulation is built using the Bevy game engine and the Rapier physics engine.

This simulation environment can also be used for RL training as we can run with run the Bevy and Rapier physics engine in headless mode allowing for many simulations to be run in parallel by not rendering to the screen. This is important as RL requires many simulations to be run to allow enough time for the agent to learn.

== Simulation Environment

==== Bevy Game Engine
The Bevy game engine is a game engine built in Rust. it is a data-driven engine that utilises entity component systems (ECS) to manage game state. This allows for a better separation of concerns and allows many features and systems to be added while keeping the codebase maintainable. It also allows for easy extensibility and modularity @Anderson2024bevyengine.

==== Rapier Physics Engine
The Rapier physics engine is built in Rust. It supports both 2D and 3D physics however we will use the 3D physics engine for this project. Most relevant to this project Rapier supports joint constraints, collision detection, and rigid body dynamics. This is important as we need to simulate the rocket's flight trajectory, engine thrust, and nozzle control @Crozet2024dimforge.


== Simulation Components

The simulation is made up of a few key components:
- Main Rocket Simulation Module
- Camera Systems
- User Interaction Systems
- Utility & Diagnostic Systems
