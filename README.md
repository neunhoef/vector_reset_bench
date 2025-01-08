# Vector reset microbenchmark for Rust

Prompt for Claude:

In a rust program, I want to reuse  a vector of u64 values of a specific
length multiple times. In the beginning, it has to be iniitlaized to all
zeros or all ones. I have a  list of positions which have been modified.
I can either  destroy the vector and  recreate it from scratch  or I can
reset the known dirty  positions in a loop. I wonder  which is faster. I
suspect  that if  many  entries  have been  touched,  it  is cheaper  to
recreate the vector, and if only  few entries are modified, it is better
to use the loop and keep the vector. Can you please write me a benchmark
program to  find out,  from which  number of  modified entries  I should
recreate the  vector? I  want to be  able to specify  the length  of the
vector  and the  number  of dirty  entries, which  should  be in  random
positions.

This is  the result that  Claude produced for  me. We went  through some
iterations to prevent  some compiler optimizations, but  no other manual
intervention was needed.

The outcome  is that one  should completely recreate the  vector already
when it was 20% full.

Simply run this with

```bash
cargo bench
```
