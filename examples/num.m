// // // 2 = (f_.x_.(f_[f_[x_]]))
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



PrintNumln[
    Sub[3][1]
]

PrintNumln[
    Mul[3][4]
]

PrintNumln[
    Mul[3][Mul[2][2]]
]



PrintNumln[
    Add[3][4]
]

PrintNumln[
    Add[3][Add[2][2]]
]



PrintNumln[
    Sub[10][4]
]

PrintNumln[
    Sub[10][Sub[6][2]]
]