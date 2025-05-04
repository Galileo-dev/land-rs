= Introduction
In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.

Landing an autonomous spacecraft or rocket is challenging, landing within meters of a predetermined target under varying atmospheric conditions is crucial @MARS2020. This precision enables fully reusable rockets, akin to aircraft refuelling and reuse. Generating optimal trajectories onboard the vehicle is not just desireable but a necessity as it's not always possible to remotely control the spacecraft in real-time e.g. Mars landing scenario @SanMartin2013 @Steltzner2014 @Way2007. Vehicles must be able to autonomous land and have their own robust and adaptive decision-making capabilities. Failure to generate and follow an optimal trajectory can result in losing the vehicle, payload and even human life. A reliable ladning system is a key factor in maintaining public trust and safety of astronauts need for future human spaceflight missions to be approved by regulatory bodies and be successful @Stein2003.

Historically on-board computational power was a major factor that dictated which algorithms can be used for landing, but with modern hardware and guidence computer architecture like that of the falcon 9 with it's triplex redundant system.


Commercial companies like SpaceX and Blue Origin have recently demonstrated landings within meters of their targets @blackmore2017.

For Mars 2020, a combination of @trn and @lvs sucessfully landed Perseverance *just 5m* from its aimpoint inside a crator covered with landing hazards, the new system greatly outperformed the mission requirments and was a key factor for immediate access to valuable geological samples @MARS2020.

Having a smaller landing ellipse allows for a site choice to be dirven by geology rather than engineering constraints, Jezero crater was chosen based on geological science merit once the ellipse was below 10x10km @GRANT2018106.

// TODO: Add more context on why precision landing is important for scientific missions on other planets and how it greatly reduces the cost of RLV's and the time it takes to reach areas of interest on other planets.

