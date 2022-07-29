### Ch2 Sorting

To measure the time consumed on sorting, I wrote a simple [program](https://github.com/SteveLauC/Algorithms-4th-Edition/tree/main/Ch2/time_measure) to demonstrate
this.

Usage:
```shell
mst

USAGE:
    mst [OPTIONS] <n> <algorithm>

ARGS:
    <n>            how many items you wanna sort
    <algorithm>    which algorithm you wanna use

OPTIONS:
    -h, --help    Print help information
    -o            use ordered input data instead of the random one
```

For example, to sort 1000 random items using insertion sort, we can:

```shell
$ ./target/debug/mst 1000 insertion
   Compiling t v0.1.0 (/home/steve/Documents/workspace/Algorithms-4th-Edition/Ch2/time_measure)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/t 1000 insertion`
Use insertion to sort 1000 numbers, consuming 2.067193ms
```
