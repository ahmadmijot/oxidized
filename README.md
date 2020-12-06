# Oxidized: Rust Tutorial

7 Dec 2020:

1. ```const``` use uppercase.
   1. ``` const ABC: type = val ```
2. shadowing vs ```mut```
   1. Shadowing can change value type while using same name.
3. Data Type:
   1. Scalar: int, float, bool, char.
4. Compound Type:
   1. Tuples: fixed length, different types.
      1. ```let tup: (i32, f64, u8) = (500, 6.4, 1)```
      2. get element using destructuring:
         ```let tup...; let (...) = tup;```
      1. get element using ```.```:
         ```let x (500, 6.4, 1);```
         ```let five_hundred = x.0;```
   1. Arrays: fixed length, must have same type. Allocated on stack.
      1. ```let a = [1,2,3,4,5];```
      2. ```let a: [i32;5] = [1,2,3,4,5];```
      3. ```let a: [3;3];```=```let a = [3,3,3];```
      4. get element using indexing:
         ```let first = a[0];```
      5. use Vec if want to grow or shrink in size.
5. Function parameter: must declare type.
6. Statement: instruction to perform action & not return a value.
7. Expression: eval to resulting value.

6 Dec 2020:

1. Error handling:
   1. expect = crashing on an error
   2. match = handling the error
2. ```parse``` = returns ```Result``` -enum that has variant ```Ok``` or ```Err```

5 Dec 2020: Starts Chapter 2: Guessing game
