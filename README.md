# Oxidized: Rust Tutorial

9 Dec 2020:
1. can only have one mut ref ```&mut```
2. This to prevent data race. Data race = similar to a race cond and happens when:
   1. Two or more pointers access the same data at the same time
   2. At least one pointers is being used to write to the data.
   3. No mechanism being used to sync access to data.

8 Dec 2020:

1. String literal = hardcoded in stack
2. String, unknown size, located at heap
3. ```let s = String::from("string literal");``~~~~`

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
      3. get element using ```.```:
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
8. Format rust: cargo fmt\

### Flow Control

1. use ```for``` for most time than ```while``` to avoid error.
2. range = ```a..b```
3. ```rev``` reverse range: ```for number in (1..4).rev(){arg}```

6 Dec 2020:

1. Error handling:
   1. expect = crashing on an error
   2. match = handling the error
2. ```parse``` = returns ```Result``` -enum that has variant ```Ok``` or ```Err```

5 Dec 2020: Starts Chapter 2: Guessing game
