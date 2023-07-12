## 🦀 memory management in rust

<br>

everything starts with the algorithms basics: understanding the difference between stack and heap, the two types of memory available in runtime. when rust starts to execute, it creates a stack that keeps track of everything happening in the execution, first main(), then the following functions, etc.

you might remember that stacks can’t hold too much data (and need fixed-size variables at compile time, i.e. static data types). so, for storing larger collections of variable-length data (dynamic data types), the program creates pointer references to the heap memory. also, recall that the heap is more expensive and slower, while the stack is as fast as its pointer can go.

for deallocation, rust neither uses an explicit free (like c/c++) nor a garbage collection (like python). instead memory is automatically returned once the owner is out of scope - a unique  feature in rust called ownership.

> 💡 “in the context of a program, scope, is the range within an item is valid.”

<br>



### ownership

rust’s ownership is a set of rules that the compiler checks at compile time (and that don’t slow down the program), enabling memory safety guarantees without needing garbage collection. this makes rust fast, as memory-managed languages would handle memory in runtime, but without the issues of unsafe memory errors.

in rust, each given object in memory can only be handled by one variable at a time, its owner. there can only be one owner at a time. if the owner goes out of scope, the value is dropped.

<br>

#### ownership is transferred by shallow copy

let’s look at a very simple first example of ownership: the string type, which is allocated on the heap (as opposed to string literals, which are immutable and their values must be known at compilation).

in the snippet below, we try to set the value of a string variable to a second variable. this will cause an error when println!() tries to reuse the value from the string (first variable):
  
<img width="615" src="https://user-images.githubusercontent.com/1130416/224515080-ffa0a583-5df8-43ea-955e-79f6d280c1a8.png">

this would print this error (some_var lost its ownership to another_var, becoming obsolete. in c/c++, the same value would be copied to the second variable):

<img width="713" src="https://user-images.githubusercontent.com/1130416/224515091-9b959c5e-191c-42ef-a89f-7770e2f945f8.png">

<br>

#### ownership is kept with deep copy (although more expensive)

a solution to keep some_var’s ownership would be creating a deep copy with clone(), i.e., making an explicit copy of the data by creating a second independent variable:

<img width="640" src="https://user-images.githubusercontent.com/1130416/224515112-35aa0ef1-bc95-4fb8-b742-62862ca093cf.png">


<br>

#### the exception for integers

however, rust does have some types that can be handled entirely in the stack (and so can be copied).

for instance, this is the case for small values such as single integers or float points (both some_var and another_var are stack-allocated (can be copied), so this example would not cause an ownership error):

<img width="499" src="https://user-images.githubusercontent.com/1130416/224515126-707f5c19-ced2-4b87-87e0-f723746ca3f3.png">









