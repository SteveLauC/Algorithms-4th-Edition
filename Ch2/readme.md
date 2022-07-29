To measure the time consumed on sorting, I wrote a simple [program](https://github.com/SteveLauC/Algorithms-4th-Edition/tree/main/Ch2/time_measure) to demostrate
this.

Usage: `cargo run N <ALGORITHM>`

where `N` is the number of items you wana sort, and `ALGORITHM` is the sorting 
algoritm you wanna use.

For example, to sort 1000 items using insertion sort, we can:

```shell
$ cargo run 1000 insertion
   Compiling t v0.1.0 (/home/steve/Documents/workspace/Algorithms-4th-Edition/Ch2/time_measure)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/t 1000 insertion`
Use insertion to sort 1000 numbers, consuming 2.067193ms
```
