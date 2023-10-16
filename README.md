# AI HW 1

CS 441: AI
Professor Bart Massey
Pedro Gonzalez

## Background

Rush Hour on an n×n board is known to be computationally difficult as n grows even when all cars are of length two. 

It is conjectured that the game is computationally difficult even when all cars are of length one.

Let's call the game with length one cars Bug Rush, to distinguish it from the more general game. (The idea is that the 1×1 cars are all VW Beetles.)

## Assignment

Write a solver for ASCII Bug Rush instances. Call your program "bugrush".

Your program may be written in any "reasonable" language that you are comfortable with. C, C++, Go, Java, Javascript, Python and Rust are all reasonable. For other languages, you may want to check first.

You may write your program on any platform you like, but your program should build and run on Linux where it will be graded: this can be a problem for code in .NET languages like C#, or in bespoke languages like Swift. You can use the Linux Lab machines to check that your stuff runs there.

Your program should read an instance in the ASCII format described above from a file whose name is supplied on the command line. Your program should then print the number of moves for a shortest solution to the instance. If the instance cannot be solved, your program should instead print "unsat".

## Project Details