# A Minimal Interpreter for the Gem Language

## Why I Created This

I recently decided to give rust a try, for fun! and also to sharpen my skills, maybe. I usually write in C++ but for this particular project I was drawn to rust, mainly because of how concise it is 

## The Syntax

I currently do not have a clear goal for how the language will behave and look like, I suppose it will just develop along with the interpreter, for now variables can only hold numbers, no string literals! and at the moment, there isnt even an complete
 interpreter, just the Lexer and Parser... anyways! 
Here’s a simple example of Gem’s syntax: We define variables with the var keyword, followed by it's name, and from there we can either assign a value or leave it as a declaration, functions are declared with the func keyword, the rest is pretty much
 C-style, except without types

```plaintext
var gx = 32;
var gz = 64;

func add(x, y) {
    x + y
}

func foo(z, x) {
    z * add(x, z)
}
