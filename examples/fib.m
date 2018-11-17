2 = Succ[1]
3 = Succ[2]
4 = Succ[3]
5 = Succ[4]
6 = Succ[5]
7 = Succ[6]
8 = Succ[7]
9 = Succ[8]


fib = n.(Head[n[
        old_pair.(
            Pair[Add[Head[old_pair]][Tail[old_pair]]][Head[old_pair]]
        )
    ][Pair[1][0]]]
)


Newln[fib[0][PipeStr]["."]]
Newln[fib[1][PipeStr]["."]]
Newln[fib[2][PipeStr]["."]]
Newln[fib[3][PipeStr]["."]]
Newln[fib[4][PipeStr]["."]]
Newln[fib[5][PipeStr]["."]]
Newln[fib[6][PipeStr]["."]]
Newln[fib[7][PipeStr]["."]]
Newln[fib[8][PipeStr]["."]]
Newln[fib[9][PipeStr]["."]]
