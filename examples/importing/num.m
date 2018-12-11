PrintNum = Num.(
    PipeFn[
        number.((number[PipeStr]["."]))
    ][Num]
)

PrintNumln = num_ln.(
    Newln[PrintNum[num_ln]]
)


NumToStr = NumToStr_A.(
    (Pred[NumToStr_A][a.(Concat["."][a])]["."])
)