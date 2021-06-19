### Ship It! 42:50 -> "add two numbers"

1. main calls playground
2. playground doesn't take any input.
3. playground returns Result. (pub enum Result)
4. Result is a type that represents either success (Ok) or failure (Err).
5. Inside playground (playground is for passing all the instructoins)
6. we make a mutable stack of type Stack
7. Stack is a struct a variable named values.
8. values is a variable-sized Vec
9. we return the "default value" for a type Stack. zero initialize the structure.
10. Default value is []. i.e., empty
11. Then we create a instructions vec![]
12. The instructions are push 1 and 2, then add
13. Instruction is an Enum
14. The variants of the Enum are Add and Push1
15. These variants are used so that we can choose their respective functionalities in the match {} for instructions
16. Add doesn't have any value, Push1 variant needs a value
17. for loop on instructions
18. for each instruction in the instructions vec, we run execute funcion (execute is to execute the instructions)
19. Here execute returns a Result type.
20. insead of taking the return to any variable, you can use the ? to check if the return is a success or error.
21. execute takes the stack and an instruction variant.
22. use match to match each variant with its function
23. for Push1, Push1 comes with a value. Use that value to push it into the vec. The value should be converted to Word type.
24. instruction is immutable. So consider dereferencing the borrow as &.
25. casting `&u8` as `u32` is invalid, so dereference the expression: `*value.
26. create a VMError enum to pass the appropriate variants of errors
27. for Add, take top two values of the stack, add them and push it back
28. pop returns you an Option. if there is error in popping, then it is an underflow, so handle it.
29. If there is error, error will be returned, or else the NULL tuple Ok(()) will be passed.
30. If there is an error from execute, the program will return there itself and stop.
31. else return Ok(())
32. match the return values of playground with the specific functionalities.
---
