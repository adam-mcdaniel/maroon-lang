Import["num"]

PutTwo = A.B.(
    PutStr[A]
)

GameState = PlayerOneLevel.PlayerOneAttack.(
    PlayerTwoLevel.PlayerTwoAttack.(
        PlayerThreeLevel.PlayerThreeAttack.(
            Pair[PlayerOneLevel][
                Pair[PlayerOneAttack][
                Pair[PlayerTwoLevel][
                Pair[PlayerTwoAttack][
                Pair[PlayerThreeLevel][
                Pair[PlayerThreeAttack][_]
                ]]]]]
        )
    )
)

P1Level = S.(
    Index[S][0]
)
P1Attack = S.(
    Index[S][1]
)

P2Level = S.(
    Index[S][2]
)
P2Attack = S.(
    Index[S][3]
)

P3Level = S.(
    Index[S][4]
)
P3Attack = S.(
    Index[S][5]
)



ModP1Level = S.N.(
    GameState[
        N
        ][
        P1Attack[S]
        ][
        P2Level[S]
        ][
        P2Attack[S]
        ][
        P3Level[S]
        ][
        P3Attack[S]
        ]
)

ModP1Attack = S.N.(
    GameState[
        P1Level[S]
        ][
        N
        ][
        P2Level[S]
        ][
        P2Attack[S]
        ][
        P3Level[S]
        ][
        P3Attack[S]
        ]
)

ModP2Level = S.N.(
    GameState[
        P1Level[S]
        ][
        P1Attack[S]
        ][
        N
        ][
        P2Attack[S]
        ][
        P3Level[S]
        ][
        P3Attack[S]
        ]
)

ModP2Attack = S.N.(
    GameState[
        P1Level[S]
        ][
        P1Attack[S]
        ][
        P2Level[S]
        ][
        N
        ][
        P3Level[S]
        ][
        P3Attack[S]
        ]
)

ModP3Level = S.N.(
    GameState[
        P1Level[S]
        ][
        P1Attack[S]
        ][
        P2Level[S]
        ][
        P2Attack[S]
        ][
        N
        ][
        P3Attack[S]
        ]
)

ModP3Attack = S.N.(
    GameState[
        P1Level[S]
        ][
        P1Attack[S]
        ][
        P2Level[S]
        ][
        P2Attack[S]
        ][
        P3Level[S]
        ][
        N
        ]
)

IncP1Level = S.(
    ModP1Level[S][Add[P1Level[S]][1]]
)

IncP1Attack = S.(
    ModP1Attack[S][Add[P1Attack[S]][1]]
)

IncP2Level = S.(
    ModP2Level[S][Add[P2Level[S]][1]]
)

IncP2Attack = S.(
    ModP2Attack[S][Add[P2Attack[S]][1]]
)

IncP3Level = S.(
    ModP3Level[S][Add[P3Level[S]][1]]
)

IncP3Attack = S.(
    ModP3Attack[S][Add[P3Attack[S]][1]]
)


DecP1Level = S.(
    ModP1Level[S][Sub[P1Level[S]][1]]
)

DecP1Attack = S.(
    ModP1Attack[S][Sub[P1Attack[S]][1]]
)

DecP2Level = S.(
    ModP2Level[S][Sub[P2Level[S]][1]]
)

DecP2Attack = S.(
    ModP2Attack[S][Sub[P2Attack[S]][1]]
)

DecP3Level = S.(
    ModP3Level[S][Sub[P3Level[S]][1]]
)

DecP3Attack = S.(
    ModP3Attack[S][Sub[P3Attack[S]][1]]
)



PlayerOne = S.(
    Pair[P1Level[S]][P1Attack[S]]
)

PlayerTwo = S.(
    Pair[P2Level[S]][P2Attack[S]]
)

PlayerThree = S.(
    Pair[P3Level[S]][P3Attack[S]]
)


ModPlayerOne = S.P.(
    ModP1Level[ModP1Attack[S][Tail[P]]][Head[P]]
)
ModPlayerTwo = S.P.(
    ModP2Level[ModP2Attack[S][Tail[P]]][Head[P]]
)
ModPlayerThree = S.P.(
    ModP3Level[ModP3Attack[S][Tail[P]]][Head[P]]
)






StrAndNum = S.N.(
    Concat[S][NumToStr[N]]    
)


PrintState = S.(
    PipeFn[pass.(Newln[
        PutTwo[
            StrAndNum["Player Three Attack: "][P3Attack[S]]
            ][
            Newln[PutTwo[
                StrAndNum["Player Three Level: "][P3Level[S]]
                ][
                Newln[Newln[PutTwo[
                    StrAndNum["Player Two Attack: "][P2Attack[S]]
                    ][
                        Newln[PutTwo[
                            StrAndNum["Player Two Level: "][P2Level[S]]
                            ][
                            Newln[Newln[PutTwo[
                                StrAndNum["Player One Attack: "][P1Attack[S]]
                                ][
                                    Newln[PutTwo[
                                        StrAndNum["Player One Level: "][P1Level[S]]
                                    ][_]]]
                                ]
                            ]]
                        ]
                    ]
                ]]
            ]
        ]
    ]])][S]
)

StartState = GameState[1][3][2][5][2][5]


