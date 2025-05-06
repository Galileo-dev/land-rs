= Implementation

In this section, we will discuss the implementation of the trajectory algorithm,the feedback guidance controller and the simulation environment.
@control_architecture shows the control architecture of the trajectory generation and how it is integrated into the guidance system.

@control_architecture

#figure(
  image("../../assets/control_architecture.png", width: 80%),
  caption: [A control architecture for trajectory generation and feedback elements. Source: @Malyuta2022],
) <control_architecture>


#include "./simulation.typ"
#pagebreak()

#include "./algorithm.typ"
#pagebreak()

#include "./guidance_controller.typ"
#pagebreak()
