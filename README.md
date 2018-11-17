# maroon-lang

maroon is my attempt at a functional programming language. I wanted to become more familiar with the Lambda Calculus, so I made this! It's very simple to use.

There are some rules though.

A function takes and returns exactly one argument. Here's an example.

```fs
Fun = a.(a)
PutStrln[a["Test"]]
```

The equals sign binds the right side of the statement to the right, the parentheses group the function body, the square brackets call a function with arguments.

## Some weird quirks

#### Stack

maroon is a stack based language, and a weird one at that. There is no differentiation between data that's already been parsed and data that hasn't. Data that's been parsed is pushed to the back of the stack, and parsed data is popped off the front. When a function is called, it pops its argument off the back.

#### Strings

Strings in maroon are very, very weird. They are just functions that push tokens onto the stack to be printed off in succession. So `"Hello world\x"` becomes `none.(Hello \_ world\x)`. When that string is printed, the function is called and the stack is printed.

Also some escape characters are required in strings.

```rust
\\ -> "\"
\x -> "!"
\lp -> "("
\rp -> ")"
\lb -> "["
\rb -> "]"
\_ -> " " // this one is automatically inserted for spaces
```

#### Flags

In addition, there are several flags used to interact with the interpreter, some are not useful in their current stage, though. Each of the useful flags has their own function to interface with it, so there is no real need for the users to touch them.

#### Only addition :(

The only supported mathmatical operations are Succ and Add :(

## Standard Library

```fs
// takes two arguments returns first
True = (True_A.(True_B.(True_A)))

// takes two arguments returns second
False = (False_A.(False_B.(False_B)))


// logical and[a][b]
And = (And_P.And_Q.(And_P [And_Q][and_p]))
// logical or[a][b]
Or = (Or_P.Or_Q.(Or_P [Or_P][or_q]))
// logical not[a][b]
Not = (Not_P.(Not_P[False][true]))
// logical nand[a][b]
Nand = Nand_A.Nand_B.(Not[And[Nand_A][nand_b]])

// if[condition][then_case][else_case]
If = If_P.(If_A.(If_B.(If_P[If_A][if_b])))

// Eq[a][b] returns a=b?
Eq = Eq_A.Eq_B.(Eq_A Eq_B @eq)
Eq = Eq_A.Eq_B.(If [Eq[Eq_A][Eq_B]][true][False])
// NotEq[a][b] returns a=/=b?
NotEq = NotEq_A.NotEq_B.(Not[Eq[NotEq_A][noteq_b]])
// logical xor[a][b]
Xor = Xor_A.(Xor_B.(And[Or[Xor_A][xor_b]][Not[Eq[Xor_A][xor_b]]]))

// Pair[a][b] is a linked list
Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z&[Pair_X][pair_y]))
// Head[Pair[a][b]] returns a
Head = First_P.(First_P[True])
// Tail[Pair[a][b]] returns b
Tail = Second_P.(Second_P[False])
// Index[Pair[a][b]][0] returns a
Index = Index_P.Index_N.(Head[Index_N[Tail][index_p]])

// ToStr[test] returns "test"
ToStr = ToStrA.( none.(ToStrA) )
// Put[test] prints test to screen
Put = PutA.(PutA @print)
// Putln[test] prints test\n to screen
Putln = PutLnA.(PutLnA @println)
// PutStr["test"] prints test to screen
PutStr = PutStrA.(PutStrA[_] @print*)
// PutStrln["test"] prints test\n to screen
PutStrln = PutStrLnA.(PutStrLnA[_] @println*)

// Newln["test"] prints \n to screen and returns "test"
Newln = NewlnA.(\\_ @println NewlnA))
// Pipe[test] prints test to screen and returns test
Pipe = PipeA.(PipeA @print_pipe)
// Pipeln[test] prints test\n to screen and returns test
Pipeln = PipelnA.(PipelnA @print_pipe \\_ @println)
// PipeStr["test"] prints test to screen and returns test
PipeStr = PipeStrA.(PipeStrA @print*pipe*)
// PipeStrln["test"] prints test\n to screen and returns test
PipeStrln = PipeStrlnA.(PipeStrlnA @print_pipe* \\* @println)
// Input["test"] returns user input
Input = InputA.(ToStr[@input])

// Rec[f][x] returns f recursively applied to f[x]
Rec = Rec_Function.Rec_Argument.(Rec_Argument Rec_Function @rec)
// Break["test"] breaks out of a Rec loop
Break = Break_A.(@break)

// Succ[n] returns Add[n][1]
Succ = Succ_N.Succ_F.Succ_X.( Succ_F[Succ_N&[Succ_F&][Succ_X&] ] )
// Add[m][n] returns m+n
Add = Plus_M.Plus_N.Plus_F.Plus_X.( Plus_M[Plus_F]Plus_N[Plus_F][Plus_X]] )
// 0 is False
0 = False
// 1 is f.x.(f[x])
1 = Succ[0]
```

## Install Rust and Compile Maroon

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
