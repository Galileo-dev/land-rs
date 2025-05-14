= Simulation

== Introduction

A core piece of this paper is to develop an environment to test the @gfold algorithm and the RL method. This environment simulates the rocket's flight and the nozzle's control. The simulation uses the Bevy game engine and the Rapier physics engine.

This simulation environment can also be used for RL training, as we can run the Bevvy and Rapier physics engines in headless mode, allowing many simulations to be run in parallel by not rendering to the screen. This is important as RL requires many simulations to allow enough time for the agent to learn.


== Simulation Components

The simulation is made up of a few key components:
- *Main Rocket Simulation Module*: Handles updating the rocket's various systems, but it only involves the engine thrust and gimbal control for now.
- *Camera Systems*: Handles the camera's position and rotation to follow the rocket and pan around the rocket.
- *User Interaction Systems*: Handles the user's input. A double-click system allows for more advanced gestures to be used.
- *Utility & Diagnostic Systems*: Displays many diagnostics from the rocket's systems to the user for debugging and monitoring purposes.

=== Main Rocket Simulation Module
#include "../../code/rocket.typ"

#figure(
  image("../../assets/sim_screenshot_01.png", width: 80%),
  caption: [Early screenshot of the simulation environment.],
)

#figure(
  image("../../assets/sim_screenshot_02.png", width: 80%),
  caption: [The final screenshot of the simulation environment, featuring the computed trajectory lines using the @sc algorithm, real-time rocket dynamics, drag, and thrust forces.],
)

== Conclusion

Developing a custom simulation environment provides a real-time testbed for @sc and @drl methods, modelling vehicle dynamics and environmental disturbances, enabling repeatable and robust tests.
