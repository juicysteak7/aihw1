# AI HW 1 - Bug Rush

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

## Project Notes

The implementation of this project is in the Rust programming language.

Assuming you have cargo and rust installed, ensure the game board file is in the root directory of the project. Run the following command to run the program (with the appropriate filename).

```cargo run some5x7.bugs```

If you dont have Rust installed [Rust Official Website - Install](https://www.rust-lang.org/tools/install)

This project provided plenty of challenge in remembering and using a lot of different algorithms and data structures. 

Once you have your data structures, your 'move or neighbors' functions built it becomes much easier to build other solutions.

After my BFS solutions I reused some of the functions like 'neighbors' to make an A* solution.

Finding good hueristics to make my A* find a better solution is where I spent a lot of time. I sometimes weighted things too much or tried to complex a heurstic and it typically made my A* search preform worse than simple heurstics like 'manhattan distance' and 'blocking cars'.

I couldn't get my A* solution just right, it succeded on most of the small boards, finding the shortest path quickly, but the large ones it took more time and more steps than bfs.