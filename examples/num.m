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


PrintNumln[
    Pred[Pred[4]]
]



// // // PRED[2]
// // // Pred[2]
// // // Putln[
// // // Tail[F[F[F[PC0]]]]
// // // PrintNumln[
// // //     Head[F[F[PC0]]]
// // // ]
// // Pred[2]
// // Tail[F[F[F[PC0]]]]
// // Head[2[F][PC0]]
// // PrintNumln[
// //     1[F][PC0]
// // ]
// // Putln[
// // Head[2[F][PC0]]
// PrintNumln[
//     5
// ]

// // Add[1][1]
// // PrintNumln[
// //     2
// // ]


// 4[v.(v ?)][m]

// Cons = 
// ToInt = ToInt_N.(ToInt_N[|.|.|][|][|])

// (v.(v.(none.(v& v& ?))))[?]
// 5[Cons][|][|]
// ToInt[5]
// Putln[
//     5 @pred
// ]