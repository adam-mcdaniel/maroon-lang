2 = Succ[1]
3 = Succ[2]
4 = Succ[3]
5 = Succ[4]
6 = Succ[5]
7 = Succ[6]
8 = Succ[7]
9 = Succ[8]
10 = Succ[9]
11 = Succ[10]
12 = Succ[11]
13 = Succ[12]


PrintNum = Num.(
    PutStr[ToNum[N]]
)

PrintNumln = num_ln.(
    Newln[PrintNum[num_ln]]
)



// PrintNumln[
//     Mod[4][3]
// ]

Even = N.(
    Eq[Mod[N][2]][0]
)

Step_Collatz = N.(
    PipeFn[pass.(PrintNumln[N])][If[Even[N]]
        [pass.(Div[N][2])]
        [pass.(Add[Mul[N][3]][1])]
    [_]]
)

Collatz = N.(
    If[Eq[N][1]]
        [pass.(PipeFn[pass.(PrintNumln[N])][Break[_]])]
        [Step_Collatz]
    [N]
)


Rec[Collatz][Mul[13][3]]