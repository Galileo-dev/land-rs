= Convex Optimisation Approach

== Introduction
The Fast Algorithm for Onboard Atmospheric Powered Descent Guidance @ChenYushu2023AFAf is an algorithm developed to efficiently compute the trajectories of a rocket during descent to course correct it to an optimal landing trajectory while considering various constraints. It does this through second order cone programming (SOCP) which is a convex optimisation problem. We will implement this algorithm in this paper and compare it against the RL method. The core problem is to account for the high computational complexity of Atmospheric Powered Descent Guidance (APDG), which must be solved onboard a spacecraft in real-time.

== Fast Algorithm for Onboard Atmospheric Powered Descent Guidance

=== Problem Formulation
The paper outlines the problem Formulation as minimising the fuel consumption while keeping safe and precise landing trajectories while under aerodynamic forces.
The problem is formulated as a Second Order Cone Programming (SOCP) optimisation.

=== SOCP Formulation
The goal is to find the optimal thrust vector $A(t)$ that minimises the fuel consumption while keeping the rocket on a safe and precise landing trajectory. The SOCP formulation is as follows:
==== Decision Variables

