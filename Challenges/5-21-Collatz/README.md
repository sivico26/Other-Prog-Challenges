# Collatz's conjecture

### The challenge

Let us describe the following process with : given a number `x`, if it is even, it is going to be divided by `2`, but if it is odd it is going to be multiplied by `3` and added `1`. The process is applied recursively over the result.

The _Collatz's conjecture_ states that for any number `x > 1`  this process will converge to `1`. The number of steps required to reach `1` is known as the **stop time**. 

Now, among the first **10 Million _natural_ numbers**, which of them has the largest __stop time__?

### The solution

As this challenge is fairly straightforward we won't detail it at much extent. The solution includes the function `steps_collatz()` which implements the process described above and the main function which applies the `steps_collatz()` to all the numbers in range and returns the one with the maximum stop time.

So the answer found is:

`The max number is 8400511 and it takes 685 steps`

You can check the source code for further details. 

To give a grasp of Rust efficiency, here is a benchmark (done with [Hyperfine](https://www.youtube.com/watch?v=CEXVKH646Zk)) of the performance of how much it takes to process the 10 MIllion numbers. 

![Benchmark](/home/sivico26/Documentos/Documents/Projects/Other_prog_languages/Other-Prog-Challenges/Challenges/5-21-Collatz/benchmark.png)