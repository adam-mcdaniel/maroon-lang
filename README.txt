Builtin Functions {
    True = (True_A.(True_B.(True_A)))
    False = (False_A.(False_B.(False_B)))
    And = (And_P.And_Q.(And_P [And_Q] [And_P]))
    Or = (Or_P.Or_Q.(Or_P [Or_P] [Or_Q]))
    Not = (Not_P.(Not_P[False][True]))
    Nand = Nand_A.Nand_B.(Not[And[Nand_A][Nand_B]])
    If = If_P.(If_A.(If_B.(If_P[If_A][If_B])))
    Eq = Eq_A.Eq_B.(Eq_A Eq_B @eq)
    Eq = Eq_A.Eq_B.(If [Eq[Eq_A][Eq_B]] [True][False])
    NotEq = NotEq_A.NotEq_B.(Not[Eq[NotEq_A][NotEq_B]])
    Xor = Xor_A.(Xor_B.(And[Or[Xor_A][Xor_B]][Not[Eq[Xor_A][Xor_B]]]))


    Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z&[Pair_X][Pair_Y]))
    Head = First_P.(First_P[True])
    Tail = Second_P.(Second_P[False])
    Index = Index_P.Index_N.(Head[Index_N[Tail][Index_P]])

    ToStr = ToStrA.( none.(ToStrA) )
    Put = PutA.(PutA @print)
    PutLn = PutLnA.(PutLnA @println)
    PutStr = PutStrA.(PutStrA[_] @print*)
    PutStrLn = PutStrLnA.(PutStrLnA[_] @println*)
    Input = InputA.(ToStr[@input])

    Succ = Succ_N.Succ_F.Succ_X.( Succ_F[Succ_N[Succ_F][Succ_X] ] )
    Plus = Plus_M.Plus_N.Plus_F.Plus_X.( Plus_M[Plus_F][Plus_N[Plus_F][Plus_X]] )
    Mult = Mult_M.Mult_N.Mult_F.Mult_X.( Mult_M[Mult_N[Mult_F]][Mult_X])
    0 = False
    1 = Succ[0]
}

The only functions you should really need are:
    True, False
    And, Or, Not,

    If: (If[condition][then][else]),
    Eq: (are these two things equal?), NotEq (are these two not things equal?),

    Put: prints a function to the screen
    PutLn: prints a function to the screen with a newline
    PutStr: prints a string to the screen
    PutStrLn: prints a string to the screen with a newline
    Input: takes a void parameter (a parameter that gets tossed) and returns user input