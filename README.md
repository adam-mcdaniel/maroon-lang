# maroon-lang

maroon is my attempt at a functional programming language. I wanted to become more familiar with the Lambda Calculus, so I made this! It's very simple to use.

## Syntax

---

### Calling a function

Functions are called the same way as in other languages, but using square brackets `[]` instead of parentheses `()`. I used this to reduce the confusion between grouping and calling functions. Also, it's very aesthetically pleasing.

```fs
PutStrln["Test"]
```

Parentheses are instead used for grouping things under one scope.

### Defining a function

The `=` operator is used to bind one side of the equals sign to the other. The left side becomes equal to the right side.

```fs
If = C.A.B.(
    C[A][B] // im a comment
)

PutStrln[
    If[False]
        ["a"]
        ["b"]
] // This will print "b"
```

A function takes and returns exactly one argument. However, you can curry functions, meaning that you can effectively take multiple arguments to a function by returning a function to take the second argument, or a third argument or so on.

```fs
2 = Succ[1] // 2 is the successor to 1
3 = Succ[2] // 3 is the successor to 2
4 = Succ[3] // 4 is the successor to 3

Exp = M.N.(
    Div[N[Mul[M]][M]][M]
) // Returns M^N

two_to_the_nth_power = Exp[2]
// Returns N.(
//     Div[N[Mul[2]][2]][2]
// )

PutStrln[
    NumToStr[two_to_the_nth_power[3]]
    // Is Div[2[Mul[2]][2]][2]
]
```

## Standard Library

---

These functions are predefined for you in the language, there's no need to define these yourself. Also, if you see something like this: `@something_here`, it's a flag to the interpreter. These are used to talk to the programming language itself. You don't need to use these flags because each of them have interfaces through which you can use them indirectly, such as the Add function.

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
Concat = Concat_A.Concat_B.(Concat_B Concat_A @concat)
// Concatenates two strings
Replace = A.B.C.(C B A @replace_string)
// (Substitutes substring B for string C) in string A

IndexStr = S.N.(S N @index_string)
// Returns a character in a string S at index N
RangeStr = S.A.B.(S B A @range_string)
// Returns a substring from a string S starting index A with length B
In = A.B.(If[B A @in_string][True][False])
// Returns True if B is in A, otherwise False

```

### Pairs (lists)

```fs
Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z[Pair_X][Pair_Y]))
// Takes two arguments and returns a pair of the two
Cons = Pair


Nil = False
// Nil is False
IsNil = N.(Eq[N][False])

Head = P.(P[True])
First = Head
// Takes a pair and returns the first object in the pair

Tail = P.(P[False])
Second = Tail
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

## Install Rust and Compile Maroon

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
