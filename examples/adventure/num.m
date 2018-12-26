Import["io"]

GetNum = Str.(StrToNum[Prompt[Str]])

2 = Succ[1]
3 = Succ[2]
4 = Succ[3]
5 = Succ[4]
6 = Succ[5]
7 = Succ[6]
8 = Succ[7]
9 = Succ[8]

PrintNum = N.(
    StrToNum[PutStrln[NumToStr[N]]]
)

Even = N.(Eq[Mod[N][2]][0])

Random = S.(
    If[Eq[S][0]][
        StrToNum["165"]
    ][
        If[Eq[S][1]][
            StrToNum["131"]
        ][
            If[Eq[S][3]][
                StrToNum["169"]
            ][
                If[Even[S]]
                    [Add[Mul[S][3]][7]]
                    [Div[Sub[S][3]][2]]
            ]
        ]
    ]
)