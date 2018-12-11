Import["num"]


fib = Fib_N.(Head[Fib_N[
        old_pair.(
            Pair[Add[Head[old_pair]][Tail[old_pair]]][Head[old_pair]]
        )
    ][Pair[1][0]]]
)


map_fib = Map_Fib_F.Map_Fib_N.(Head[Map_Fib_N[
        old_pair.(
            Pair[Add[Map_Fib_F[Head[old_pair]]][Tail[old_pair]]][Head[old_pair]]
        )
    ][Pair[1][0]]]
)


