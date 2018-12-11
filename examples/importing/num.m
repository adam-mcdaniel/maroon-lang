
NumToStr = NumToStr_A.(
    Pred[NumToStr_A]
    [
        thingy.(Concat["."][thingy])
    ]["."]
)

PrintNum = PipeFn[number.((number[PipeStr]["."]))]

PrintNumln = num_ln.(
    Newln[PrintNum[num_ln]]
)
