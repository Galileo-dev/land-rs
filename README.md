# land-rs

A suite of programs to land rockets.

## Land Sim

a simulation made using the Bevy game engine to simulate all of the basic physics of a mass body in the gravitational influence of a planet. This is used to simulate the landing of a rocket on a planet.

- Provides an interface to control the simulation from an external program in this case `land-core`.

### Configurations

there are many configurations I would like to test

- [ ] 1. simulation of a single rocket landing on a planet
- [ ] 2. simulation of multiple rockets landing on a planet in a single simulation in a tight formation
- [ ] 3. simulation of a single rocket experiencing an anomaly (e.g. engine failure, loss of control, etc.)
- [ ] 4. simulation of a single rocket recovering from a bad landing

## Land Core

a library that contains all of the core logic for landing a rocket on a planet. This is used by the land-sim program to simulate the landing of a rocket on a planet.

Different Algorithms are available to control the rocket:

- GFOLD: Convex Optimization based algorithm based on [Flight Testing of Trajectories Computed by G-FOLD: Fuel Optimal Large Divert Guidance Algorithm for Planetary Landing](https://www.researchgate.net/publication/236334043_Flight_Testing_of_Trajectories_Computed_by_G-FOLD_Fuel_Optimal_Large_Divert_Guidance_Algorithm_for_Planetary_Landing)
- SAA: Stupid Autonomous Algorithm (Basically a PID controller with some very basic linear motion equations)

## Land KRPC

a program that uses the [KRPC library](https://krpc.github.io/krpc/) to connect to the KRPC server and control the rocket in the simulation. This is used to test the land-core library with [kerbal space program](https://wiki.kerbalspaceprogram.com/wiki/Main_Page).

- uses the same interface as Land Sim to connect to the land core.

## Land Protocol

A basic protocol designed to communicate between the rocket and the program that is controlling the guidance of the rocket.

- This protocol will use a websockets to communicate between the rocket and the guidance program.
