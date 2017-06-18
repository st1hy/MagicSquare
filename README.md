# MagicSquare

A naive approach to searching magic 3x3 square with squared numbers using MonteCarlo method.

Written in several languages as an exercise and as a kind of a benchmark.


 Lang                | Single thread	| Multi thread (4 cores)
 ---                 | :---:            | :---:
 Python 3.6          |   44.5 kT/s	    |   44.0 kT/s
 Java 1.8            |    4.4 MT/s	    |   9.9 MT/s
 Kotlin	 (JVM 1.8)   |    4.4 MT/s	    |   10 MT/s
 Rust 1.17	         |    3.3 MT/s	    |   11.1 MT/s
 Rust (nightly 1.18) |    4.0 MT/s	    |   13.2 MT/s
 Rust 1.18           |    3.3 MT/s      |   11.6 MT/s
 Rust (nightly 1.19) |    4.0 MT/s	    |   14.1 MT/s
 C++ (gnu++14 -O3)   |    3.6 MT/s      |   12.4 MT/s


 kT/s -     1 000 squares tried per second

 MT/s - 1 000 000 squares tried per second

