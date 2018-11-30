2 = Succ[1]
3 = Succ[2]
4 = Succ[3]
5 = Succ[4]
6 = Succ[5]
7 = Succ[6]
8 = Succ[7]
9 = Succ[8]
10 = Succ[9]

PrintNum = Num.(
    PipeFn[
        number.((number[PipeStr]["."]))
    ][Num]
)

PrintNumln = num_ln.(
    Newln[PrintNum[num_ln]]
)

fib = n.(Head[n[
        old_pair.(
            Pair[Add[PrintNumln[Head[old_pair]]][Tail[old_pair]]][Head[old_pair]]
        )
    ][Pair[1][0]]]
)


fib[10]
