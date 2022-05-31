# Coding Drag Race
Drag racing vanilla programming languages at various computational tasks.

## Rules:

* Implementations cannot use libraries for core functionality, excluding the language's standard library.
* Each implementation ought to fulfill the same usage, e.g. accept the same arguments, return identical output, etc.

## Cellular Automata

### Performance tests
All performance tests were run on my local machine running Linux Mint, and times are the average of 3 runs.
You will likely have different times on your machine, but comparisons between languages should be similar.
The command used to measure performance is provided for convenience.
I suspect that the compiled languages are so fast that this actually mostly tests the speed of system calls, so this may not be the best test for performance. 

1. Python
   1. `time main.py --width=256 --rule=30 --length=102400`
   2. Completed in 13.5 seconds
2. Go
   1. `time main -width=256 -rule=30 -length=102400`
   2. Completed in 7.63 seconds
3. Rust
   1. `time rust -r=30 -l=102400 -w=256`
   2. Completed in 10.2 seconds

## Future projects:
* Bouncing Ball https://www.youtube.com/watch?v=kLj-H1K317U
* Mandelbrot renderer
