// // // 2 = (f_.x_.(f_[f_[x_]]))
2 = Succ[1]
3 = Succ[2]
4 = Succ[3]
5 = Succ[4]

PrintNum = Num.(
    PipeFn[
        number.((number[PipeStr]["."]))
    ][Num]
)

PrintNumln = num_ln.(
    Newln[PrintNum[num_ln]]
)