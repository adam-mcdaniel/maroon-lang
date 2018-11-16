// Boolean logic
True = (True_A.(True_B.(True_A)))
False = (False_A.(False_B.(False_B)))

And = (And_P.And_Q.(And_P [And_Q] [And_P]))
Or = (Or_P.Or_Q.(Or_P [Or_P] [Or_Q]))
Not = (Not_P.(Not_P[False][True]))
Xor = Xor_A.(Xor_B.(And[Or[Xor_A][Xor_B]][Not[Eq[Xor_A][Xor_B]]]))
Nand = Nand_A.Nand_B.(Not[And[Nand_A][Nand_B]])

// Control flow
Loop = Loop_Function.((LOOP_A.(LOOP_A[LOOP_A]))[LOOP_A.(Loop_Function[?] LOOP_A[LOOP_A])])
If = If_P.(If_A.(If_B.(If_P[If_A][If_B])))
Void = (v.)

// // Eq
Eq = Eq_A.Eq_B.(Eq_A Eq_B @eq)
Eq = Eq_A.Eq_B.(If [Eq[Eq_A][Eq_B]] [True][False])
NotEq = NotEq_A.NotEq_B.(Not[Eq[NotEq_A][NotEq_B]])

// // Pairs
Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z&[Pair_X][Pair_Y]))
Head = First_P.(First_P[True])
Tail = Second_P.(Second_P[False])


// STDIN / STDOUT
Print = (Print_A.(Print_A @print))
// Print* = (Print*_A.(Print*_A[*] @print*))
Println = (Println_A.(Println_A @println))
// Println* = (Println*_A.(Println*_A[*] @println*))
// Prompt = Prompt_A.(Prompt_A @print _ @print @input)

// // Math
// IsZero = IsZero_N.(IsZero_N[IsZero_X.False][True])

// // Pred = Pred_N.Pred_F.Pred_X.( Pred_N[G.H.(H&[G[Pred_F]])][U.Pred_X][U.U&] )


// Succ = Succ_N.Succ_F.Succ_X.( Succ_F[Succ_N[Succ_F][Succ_X] ] )
// Minus = Minus_M.Minus_N.(Minus_N[Pred][Minus_M])
// Plus = Plus_M.Plus_N.Plus_F.Plus_X.( Plus_M[Plus_F][Plus_N[Plus_F][Plus_X]] )
// Mult = Mult_M.Mult_N.Mult_F.Mult_X.( Mult_M[Mult_N[Mult_F]][Mult_X])

// 0 = False
// 1 = Succ[0]
// 2 = Succ[1]
// 3 = Succ[2]
// 4 = Succ[3]
// 5 = Succ[4]
// 6 = Succ[5]
// 7 = Succ[6]

// F__ = F__P.(Pair[Tail[F__P]][Succ[Tail[F__P]]])
// PC0 = Pair[False][False]
// Pred = Pred_N.(Head[Pred_N[F__][PC0]])


// LessEq = LessEq_M.LessEq_N.(IsZero[Minus[LessEq_M][LessEq_N]])
// NumEq = NumEq_M.NumEq_N.(And[LessEq[NumEq_M][NumEq_N]][LessEq[NumEq_N][NumEq_M]])

// Index = Index_P.Index_N.(Head[Index_N[Tail][Index_P]])









// True = True_A.True_B.True_A
// False = False_A.False_B.False_B


// And = (And_P.And_Q.(And_P [And_Q] [And_P]))
// Or = (Or_P.Or_Q.(Or_P [Or_P] [Or_Q]))
// Not = (Not_P.(Not_P[False][True]))
// Xor = Xor_A.(Xor_B.(And[Or[Xor_A][Xor_B]][Not[Eq[Xor_A][Xor_B]]]))
// Nand = Nand_A.Nand_B.(Not[And[Nand_A][Nand_B]])



// Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z&[Pair_X][Pair_Y]))
// Head = First_P.(First_P[True])
// Tail = Second_P.(Second_P[False])
Index = Index_P.Index_N.(Head[Index_N[Tail][Index_P]])

Succ = Succ_N.Succ_F.Succ_X.( Succ_F[Succ_N[Succ_F][Succ_X] ] )
// Add = Plus_M.Plus_N.Plus_F.Plus_X.( Plus_M[Plus_F][Plus_N[Plus_F][Plus_X]] )

// Pred = Pred_N.Pred_F.Pred_X.( Pred_N[G.H.(H&[G[Pred_F]])][U.Pred_X][K_.K_&] )
// F__ = F__P.(Pair[Tail[F__P]][Succ[Tail[F__P]]])
// PC0 = Pair[False][False]
// Pred = Pred_N.(Head[Pred_N&[F__][PC0]])



// 0 = False
// 1 = Succ[0]
// 2 = Succ[1]
// 3 = Succ[2]
// 4 = Succ[3]
// 5 = Succ[4]
// 6 = Succ[5]
// 7 = Succ[6]



// my_list = Pair[a][
//     Pair[b][
//         Pair[c][
//             Pair[d][
//                 Pair[e][
//                     f
//                 ]
//             ]
//         ]
//     ]
// ]


ToStr = ToStrA.( none.(ToStrA) )
PutStrLn = PutStrLnA.(PutStrLnA[_] @println*)
PutStr = PutStrA.(PutStrA[_] @print*)
Input = InputA.(ToStr[@input])






// Println[ \( ]

// Prompt = Prompt_A.(PutStr[Prompt_A] Input[_])
PutStrLn["tell me your name!"]

NAME = Input[_]


PutStr["'"]
PutStsr[NAME]
PutStrLn["'"]

PutStrLn[
    If[Eq[NAME]["adam"]]
        [
            "hey me!"
        ][
            If[Eq[NAME]["addison"]][
                "hey addison!"
            ][
                ":\lp"
            ]
        ]
]



















// PutStrLn["hey jude"]

// // PutStr["hmm"]
// NAME = Input[">>>"]
// // PutStrLn[NAME]

// Println[
//     Eq[NAME]["test"]
// ]


// Add = Add_M.Add_N.Add_F.Add_X.(Add_M[Succ][Add_N[Add_F][Add_X]])

// Index[my_list][Add[2][2]]
// Println[
//     Index[Pair[a][
//         Pair[b][c]
//     ]]
// ]

// Print[test]
// // Print[_]
// Print[me]

// "im a string!\n" @println*

// Println[ a.(_) ]


// FirstHalf = A.(Println[A] SecondHalf[A])

// Loop = Loop_F.Loop_Input.(Loop_F[Loop_Input])

// Y = Y_f.( (Y_x.(Y_f(Y_x[Y_x])))[(Y_x.(Y_f(Y_x[Y_x]))] )

// Test = Test_A.(Println[Test_A] Test_A)


// Println[
//     Y[Test]
//     ]


// TestMe = A_o.(
//     Did.(
//         I_work
//         ))

// Println[TestMe]


// Zero = (v.)

// Println*[
//     a.(hmm Zero[_])
// ]




















// Println[
//     Tail[Index[
//         Pair[a][
//             Pair[b][
//                 Pair[Pair[True][False]][
//                     Pair[d][-]]]]
//     ][2]]
// ]

// Println[Pred]
// two = Pred[3]
// two[a.(Println[test] a)] [i.i]
    
// one = Pred[two]
// Println[two]
// Println[-]
// Println[one]
// Plus[2][1]
//     [a.(Println[test] a)][i.i]
// Pred[Pred[Succ[4]]][a.(Println[test] a)][i.i]
// Pred[Succ[Pred[3]]][a.(Println[test] a)][i.i]

// Println[ If [Eq[A][A]] [true!] [false!] ]

// Println[
//     7[a.a][test]
// ]



// Println[Hi!]
// Println*[none.(Whats _ your _ name?)]
// NAME = Prompt[>]
// Println* [
//     If [Eq[NAME][Adam]]
//         [none.(Yay! _ :D)]
//         [If [Eq[NAME][Addison]]
//                 [none.(hi!! _ :D)]
//                 [If [Eq[NAME][Logan]]
//                     [none.(Well _ howdy!)]
//                     [none.(I _ dont _ know _ you _ :|)]]]
// ]



// S = S_X.S_Y.S_Z.((S_X&[S_Z])[(S_Y&[S_Z])])
// K = K_Q.K_I.K_Q
// I = I_I.I_I&
// Println [
//     S[I][I][True][True]
// ]
// Println [
//     K[S[K]][a.a]
// ]




// 1884, Berlin conference
// European leaders decided that the first to set up an outpost on an
// area of land would own it. (in africa)
// 
// mid 1600s, Dutch farmers called Boers settled in Cape Colony
// The Boers built Cape Town as a supply station.
// 
// In the 1700s, the Dutch herders and ivory hunters began to move North.
// The British then acquired Cape Colony in the early 1800s.
// 
// Southern Africa
// The Zulus were skilled and oganized fighters
// Shaka used his power and fought against Zulu enemies
// 
// The Zulus also fought against Boers from North Africa
// 
// In 1879, the Zulus wiped out British at Battle of Isandlwana
// However, British weaponry overtook the Zulus
// at the battle of Rorke's Drift
// 
// In 1910, with southern Africa secure, the British established the
// Republic of South Africa and instituted apartheid.
// 
// Apartheid - Government policy requiring segregation.
// 
// After the slave trade was outlawed, abolitionists
// in the US promoted idea of returning freed slaves to africa
// In early 1800s, Monroe helped free slaves settle in Liberia.
// 
// The former slaves formed Monrovia in his honor.
// 
// Ethiopia regained freedom with a military uprising.