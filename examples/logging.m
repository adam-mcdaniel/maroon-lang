Log = Log_A.Log_B.(
    PipeFn[c.(PipeFn[a.(PipeStrln[Log_B])][PipeStr[Log_A]])][Log_B]
)

Info = Info_A.(Log["=( INFO )====> "][Info_A])
Debug = Debug_A.(Log["=| DEBUG |===> "][Debug_A])
Warning = Warning_A.(Log["={ WARNING }=> "][Warning_A])
Error = Error_A.(Log["=[ ERROR ]===> "][Error_A])

// Info["hey jude"]
// Debug["hey jude"]
// Warning["hey jude"]
// Error["hey jude"]

Good = True
Bad  = Bad_A.Bad_B.Bad_B


PrintNum = Num.(
    PipeFn[
        number.((number[PipeStr]["."]))
    ][Num]
)

PrintNumln = num_ln.(
    Newln[PrintNum[num_ln]]
)

2 = Succ[1]
3 = Succ[2]
4 = Succ[3]
5 = Succ[4]

Plus = A.B.(
    Pair[Add[A][B]][Good]
)

ToInt = Str.(
    If[Eq[Str]["0"]][
        0
    ]
    [
        If[Eq[Str]["1"]][
            1
        ]
        [
            If[Eq[Str]["2"]][
                Succ[1]
            ]
            [
                If[Eq[Str]["3"]][
                    Succ[Succ[1]]
                ]
                [
                    If[Eq[Str]["4"]][
                        Succ[Succ[Succ[1]]]
                    ]
                    [
                        If[Eq[Str]["5"]][
                            Succ[Succ[Succ[Succ[1]]]]
                        ]
                        [
                            If[Eq[Str]["6"]][
                                Succ[Succ[Succ[Succ[Succ[1]]]]]
                            ]
                            [
                                If[Eq[Str]["7"]][
                                    Succ[Succ[Succ[Succ[Succ[Succ[1]]]]]]
                                ]
                                [
                                    If[Eq[Str]["8"]][
                                        Succ[Succ[Succ[Succ[Succ[Succ[Succ[1]]]]]]]
                                    ]
                                    [
                                        If[Eq[Str]["9"]][
                                            Succ[Succ[Succ[Succ[Succ[Succ[Succ[Succ[1]]]]]]]]
                                        ]
                                        [
                                            Bad
                                        ]
                                    ]
                                ]
                            ]
                        ]
                    ]
                ]
            ]
        ]
    ]
)


SafeToInt = StrN.(
    If[NotEq[ToInt[StrN]][Bad]]
        [
            a.(PrintNumln[a])
        ]
        [
            a.(PipeFn[a.(Error["Not a single digit integer"])][0])
        ][ToInt[StrN]]
)

PutStr[">> "]
int = SafeToInt[Input[_]]

Debug[ToStr[int]]

// result = Plus[2][3]
// (If[Tail[result]]
//     [
//        a.(PrintNumln[
//            Head[a]
//        ])
//     ]
//     [
//         a.(Info["Could not add"])
//     ])[
//         result
//     ]