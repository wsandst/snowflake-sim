# Snowflake Simulation for the web
This project implements a Snowflake Simulation using Reiters model. The underlying simulation is written in Rust using WebAssembly and the interactive part is created using Svelte for the web. There is rudimentary support for replaying simulations by URL, which can be accessed by
pressing the share button in the interface.

## URL parameters
In addition to the parameters visible in the interface, some settings can only be modified through URL parameters. For example,
`www.wsandst.com/snowflakes/?size=200x200&color=ff0000` makes the simulation use a 200x200 grid size and a snowflake color of red.  
Below all the available parameters are listed.  
`color=[HEX]`  
`size=[WIDTHxHEIGHT]`  
`seed=[SEED]`



## Resources
[Reiters Model, A local cellular model for snow crystal growth](http://www.patarnott.com/pdf/SnowCrystalGrowth.pdf)  
[On the Modeling of Snowflake Growth Using Hexagonal Automata](https://math.mit.edu/research/highschool/primes/materials/2014/Li-Jessica.pdf)