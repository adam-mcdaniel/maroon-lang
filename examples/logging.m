Log = Log_A.Log_B.(
    PipeFn[c.(PipeFn[a.(PipeStrln[Log_B])][PipeStr[Log_A]])][Log_B]
)

Info = Info_A.(Log["=( INFO )====> "][Info_A])
Debug = Debug_A.(Log["=| DEBUG |===> "][Debug_A])
Warning = Warning_A.(Log["={ WARNING }=> "][Warning_A])
Error = Error_A.(Log["=[ ERROR ]===> "][Error_A])


Good = True
Bad  = Bad_A.Bad_B.Bad_B


PrintNum = Num.(
    PipeFn[pass.(PutStr[NumToStr[Num]])][Num]
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


SafeToInt = StrN.(
    If[NotEq[ToInt[StrN]][Bad]]
        [
            a.(PrintNumln[a])
        ]
        [
            a.(PipeFn[a.(Error["Not a single digit integer"])][0])
        ][StrToNum[StrN]]
)

PutStr[">> "]
int = SafeToInt[Input[_]]

Debug[
    NumToStr[int]
    ]
