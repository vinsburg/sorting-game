# Sorting Game

The sorting game is comprised of `stacks` of varying sizes, and of several `kinds`. Each `kind` has multiple `units`, scattered across the `stacks`.
You may move `units` of a `kind` from one `stack` to another if the top `units` in both `stacks` are of the same `kind`, and if there is room in the second `stack` for all said `units` from the first `stack`.
The goal is for all `stacks` to be either empty, or contain all `units` of a single `kind`.

```
|**************|
| Sorting Game |
****************
Stage - 2
Turn - 1

 1:  1  2  3 __ __ 
 2:  5  5  3  3  4 
 3:  6  7  8  2  8 
 4:  9  7  7 __ __ 
 5:  2  7  1 10 __ 
 6:  9  5  5  3  9 
 7:  7  3 10  9 __ 
 8: __ __ __ __ __ 
 9:  6  6  1 __ __ 
10:  5  8  6 __ __ 
11:  8  4  9 __ __ 
12: 10 10  8  6  1 
13:  2  4  1 10 __ 
14:  4  2  4 __ __ 

Select stacks to move from and to (e.g., '2 3'). Type 'r' to reset the stage:
```