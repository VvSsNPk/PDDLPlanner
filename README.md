# Readme for Escape Wumpus Cave using PDDL planners

The language used for this assignment is rust and you will find the code in the source folder.

> you need to have fast-downward planner directory inside the project folder for the program to work.

> It need be of the name fast-downward-23.06 with a fast-downward.py file insdie the directory this script invokes the planner.

As for all of my previous assignments my project needs cargo to compile and run. 
I did not automate the verification process like my previous assignment you need to manually run the verification script to check my solutions

To build the program, first build it using the following command

````cmd
cargo build --release
````

To run the program, you need the following commandl

````cmd 
cargo run --release
````
>> Since the program only handles input and output. Most of the work is done by the planner. This program is just a parser.

The program also dumps the pddl problem files to check,
by default, the program points out to the maps folder that contains 
the maps for the wumpus cave, so if you run the program, it will give you solutions to check.


Now I will go through briefly the solution summary [here](Solution-summary.md)


