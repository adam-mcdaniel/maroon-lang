List = List_X.List_Y.(List_Z.(List_Z[List_X][List_Y]))

l1 = List["A"][List["B"]["C"]]

PutStrln[
    Tail[Tail[l1]]
]

PutNum = Num.(
    PipeFn[
        number.((number[PipeStr]["."]))
    ][Num]
)

PutNumln = Num.(
    Newln[PutNum[Num]]
)

2 = Succ[1]
3 = Succ[2]


PutNumln[3]