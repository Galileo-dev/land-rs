= Progress Update

== Work Done to Date
- Conducted initial research on the @gfold algorithm and convex optimization by reviewing research papers. (Due to current gaps in understanding, I have enrolled in Stanford EE364A for a deeper dive @boyd_vandenberghe_ee364a.)
- Set up the development environment and project structure using a Cargo workspace, enabling modularity by splitting the project into separate crates.
- Created a fully operational simulation environment using the Bevy game engine and the Rapier physics engine. The prototype includes a basic wireframe renderer that displays the rocket, ground, and nozzle. See @sim_screenshot_01.

#figure(
  image("../assets/sim_screenshot_01.png", width: 80%),
  caption: [Screenshot of the simulation environment prototype.],
) <sim_screenshot_01>
