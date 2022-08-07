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


Sorting algorithm performance overview:
```shell
$ ./mst 10000 selection
Use <selection sort> to sort 10000 random numbers, consuming 52.548673ms

$ ./mst 10000 selection -o
Use <selection sort> to sort 10000 ordered numbers, consuming 51.823085ms

$ ./mst 10000 insertion
Use <insertion sort> to sort 10000 random numbers, consuming 22.921095ms

$ ./mst 10000 insertion -o
Use <insertion sort> to sort 10000 ordered numbers, consuming 21.507µs

$ ./mst 10000 shell
Use <shell sort> to sort 10000 random numbers, consuming 1.246734ms

$ ./mst 10000 shell -o
Use <shell sort> to sort 10000 ordered numbers, consuming 316.868µs
```
