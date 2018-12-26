# maroon-lang

maroon is my attempt at a functional programming language. I wanted to become more familiar with the Lambda Calculus, so I made this! It's very simple to use.

There are some rules though.

A function takes and returns exactly one argument. Here's an example.

```fs
Fun = a.(a)
PutStrln[Fun["Test"]]
```

The equals sign binds the left side of the statement to the right, the parentheses group the function body, the square brackets call a function with arguments.

## Some weird quirks

#### Stack

maroon is a stack based language, and a weird one at that. There is no differentiation between data that's already been parsed and data that hasn't. Data that's been parsed is pushed to the back of the stack, and data to be parsed is popped off the front. When a function is called, it pops its argument off the back.

#### Flags

In addition, there are several flags used to interact with the interpreter, some are not useful in their current stage, though. Each of the useful flags has their own function to interface with it, so there is no real need for the users to touch them.

#### Math

The only supported mathmatical operations are Succ, Add, and Pred :(

# Standard Library
---

### Import

```fs
Import = Import_A.(Import_A @import)
```

Import runs a script by name and imports all of its assignments.

#### Example
```fs
Import["test"] // will import test.m
```


### Logic

```fs
True = A.B.(A)              // Takes two arguments, returns first
False = A.B.(B)             // Takes two arguments, returns second
And = P.Q.(P[Q][P])         // A AND B
Or = P.Q.(P[P][Q])          // A OR B
Not = P.(P[False][True])    // NOT A
Nand = A.B.(Not[And[A][B]]) // A NAND B
If = C.A.B.(C[A][B])        // Returns A if C is True. Else, return B.

Eq = A.B.(A B @eq)
Eq = A.B.(C.A.B.(C[A][B]) [Eq[A][B]] [True][False])
NotEq = A.B.(Not[Eq[A][B]])
// Ignore the weird stuff, just know that Eq[A][B] is A == B

Xor = A.(B.(And[Or[A][B]][Not[Eq[A][B]]])) // A XOR B
```


### IO
```fs
ToStr = A.( none.(A) )
// Converts an Identifier to a string
// For example, ToStr[Hello] returns "Hello"

ToVal = ToValA.( ToValA[none] )
// Reverses ToStr


Newln = NewlnA.(\\_ @println NewlnA)
// Takes an argument, prints new line, returns argument

Pipe = PipeA.(PipeA @print_pipe)
// Takes an argument, prints the argument, returns argument

Pipeln = PipelnA.(PipelnA @print_pipe \\_ @println)
// Takes an argument, prints the argument and a new line, returns argument

PipeStr = PipeStrA.(PipeStrA @print_pipe*)
// Takes a String, prints the String, returns String

PipeStrln = PipeStrlnA.(PipeStrlnA @print_pipe* \\_ @println)
// Takes a String, prints the String and a new line, returns String

Input = InputA.(ToStr[@input])
// Takes an argument and returns user input

// Other names for pipe functions, more intuitive to user
Put = Pipe
Putln = Pipeln
PutStr = PipeStr
PutStrln = PipeStrln
```

### Strings
```fs
// Not much here yet, there will be more :)

Concat = Concat_A.Concat_B.(Concat_B Concat_A @concat)
// Concatenates two strings 
```


### Pairs (lists)

```fs
Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z[Pair_X][Pair_Y]))
// Takes two arguments and returns a pair of the two

Head = First_P.(First_P[True])
// Takes a pair and returns the first object in the pair

Tail = Second_P.(Second_P[False])
// Takes a pair and returns the second object in the pair

Index = Index_P.Index_N.(Head[Index_N[Tail][Index_P]])
// Takes a linked pair (a chain of pairs within pairs)
// and returns the Nth recursion depth
```


### Kind of IO?

```fs
PipeFn = Pipe_function.Pipe_x.(v.()[Pipe_function[Pipe_x]] Pipe_x)
// Takes a function and a value, and calls the function with the value
// Then returns the original value.
// Used purely to call the function for its side effects

Exit = Exit_A.(@exit)
// Exits program

Eval = S.(S @eval)
// Evaluates string

Rec = Rec_Function.Rec_Argument.(Rec_Argument Rec_Function @rec)
// Recursively calls function, like a church numeral infinity
// (If you dont know what church numerals are, google them)
Break = Break_A.(@break)
// Takes an argument and breaks from Rec function
```

### Math

```fs
Succ = N.(N @succ)
// N + 1
Pred = N.(N @pred)
// N - 1
Add = M.N.(N M @add)
// M + N
Sub = M.N.(N M @sub)
// M - N
Mul = M.N.(N M @mul)
// M * N
Div = M.N.(N M @div)
// M / N
Mod = M.N.(N M @mod)
// M % N

StrToNum = N.(N @to_fun)
// Converts "1" to the church numeral for 1, or "99" to the church numeral for 99

NumToStr = N.(N @num)
// Converts the church numeral for 1 to "1", or the church numeral for 99 to "99"


0 = F.X.( X)
// Definition of Zero
1 = Succ[0]
// One comes after Zero
// You have to define the rest of the numbers yourself!
```

# Install Rust and Compile Maroon
---

It's super easy.

For \*nix

```bash
cd ~/Desktop
curl https://sh.rustup.rs -sSf | sh
git clone https://github.com/adam-mcdaniel/maroon-lang
cd maroon-lang
cargo build --release
./target/release/maroon examples/hello_world.m
```

The path to the maroon executable is "target/release/maroon". It's statically linked, so you can move it where ever youd like without worrying about dependencies :)
