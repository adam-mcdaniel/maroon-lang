Import["num"]


fib = Fib_N.(Head[Fib_N[
        old_pair.(
            Pair[Add[Head[old_pair]][Tail[old_pair]]][Head[old_pair]]
        )
    ][Pair[1][0]]]
)


print_fib = Print_Fib_N.(Head[Print_Fib_N[
        old_pair.(
            Pair[Add[PrintNumln[Head[old_pair]]][Tail[old_pair]]][Head[old_pair]]
        )
    ][Pair[1][0]]]
)


