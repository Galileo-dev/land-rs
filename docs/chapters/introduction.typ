= Introduction
In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.

Landing an autonomous spacecraft or rocket is challenging; landing within meters of a predetermined target under varying atmospheric conditions is crucial @MARS2020. This precision enables fully reusable rockets, akin to aircraft refuelling and reuse. Generating optimal trajectories onboard the vehicle is not just desirable but necessary, as it is not always possible to remotely control the spacecraft in real-time, e.g. Mars landing scenario @SanMartin2013 @Steltzner2014 @Way2007. Vehicles must be able to autonomously land and have their own robust and adaptive decision-making capabilities. Failure to generate and follow an optimal trajectory can result in losing the vehicle, payload and even human life. A reliable landing system is a key factor in maintaining public trust and the safety of astronauts, which is necessary for future human spaceflight missions to be approved by regulatory bodies and be successful @Stein2003.

Historically, onboard computational power was a significant factor that dictated which algorithms could be used for landing, but with modern hardware and modern guidance, computer architecture like the Spacex Falcon 9 or Starship with its triplex redundant system.

Commercial companies like Spacex and Blue Origin have recently demonstrated landings within meters of their targets @blackmore2017 @Szmuk2016.

For Mars 2020, a combination of @trn and @lvs successfully landed Perseverance *just 5m* from its aimpoint inside a crater covered with landing hazards, the new system significantly outperformed the mission requirements and was a key factor for immediate access to valuable geological samples @MARS2020.

A smaller landing ellipse allows a site choice to be driven by geology rather than engineering constraints. Jezero crater was chosen based on geological science merit once the ellipse was below 10x10km @GRANT2018106.

Percision landing enables a critical turning point in the cost of space exploration and payload delivery. @rlv:pl significantly reduce the operational and refurbishment costs, and significantly improve economic viability of space flight. Reusability focuses on reducing the the costs associated with the recovery, inspection and refurbishment of the vehicle, mainly with high wear components like engine, avionics and tanks. Percision landing significantly drives down turn around time as the logistics of recovery and refurbishment are significantly reduced or eliminated @webb2016economics.


// TODO: Add more context on why precision landing is important for scientific missions on other planets and how it dramatically reduces the cost of RLV and the time it takes to reach areas of interest on other planets.
