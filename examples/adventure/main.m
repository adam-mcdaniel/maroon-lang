Import["engine"]

// Main = N.(
//     StrToNum[
//         PutStrln[
//             NumToStr[Random[N]]
//         ]
//     ]
// )

// Seed = GetNum["Enter a seed: "]

// Rec[Main][Seed]





Chain = A.B.(
    Pair[B][A]
)

Monster = Name.Health.Treasure.(
    Chain[
        Chain[
            Chain[
                _
            ][
                Name
            ]
        ][
            Health
        ]
    ][
        Treasure
    ]
)

Name = Monster.(
    Index[Monster][2]
)

Health = Monster.(
    Index[Monster][1]
)

Treasure = Monster.(
    Index[Monster][0]
)

PrintMonster = Monster.(
    PipeFn[pass.(Newln[PutTwo[
        StrAndNum[" Treasure: "][Treasure[Monster]]
    ][
        PutTwo[
            StrAndNum[" - Health: "][Health[Monster]]
        ][
            PutTwo[
                Name[Monster]
            ][
                _
            ]
        ]
    ]])][Monster]
)

Cards = Chain[
            Chain[
                Chain[
                    Chain[
                        Chain[
                            _
                        ][
                                Monster["Cthulu"][9][4]
                        ]
                    ][
                            Monster["Floating Nose"][4][2]
                    ]
                ][
                        Monster["Gnome"][1][1]
                ]
            ][
                Monster["Halfling"][7][3]
            ]
        ][
            Monster["Cultist"][5][2]
        ]



DrawCard = N.(
    Index[Cards][Mod[N][5]]
)


Fight = M.P.(
    If[Eq[Sub[Health[M]][Tail[P]]][0]]
        [pass.(PipeFn[pass.(Newln[Newln[PutTwo[" was defeated!"][PutTwo[Name[M]][_]]]])]
            [
                Pair[Add[Head[P]][1]][Add[Tail[P]][Treasure[M]]]
            ]
        )]
        [pass.(Pair[Sub[Head[P]][1]][Tail[P]])][_]
)

// Main = N.(
//     PipeFn[N.(
//             PrintMonster[DrawCard[N]]
//         )][Random[N]]
// )

// Seed = GetNum["Enter a seed: "]

PutStrln["Entering the dungeon..."]
PutStrln["It's dark and eerie. With only the light of your torch, you can barely see in front of you."]
PutStrln["Here are the players' stats."]

// PrintState[StartState]

Main = SeedAndState.(
    Newln[Pair[Random[Head[SeedAndState]]][
            PrintState[ModPlayerOne[Tail[SeedAndState]]
                [
                    Fight[PrintMonster[DrawCard[Head[SeedAndState]]]]
                        [PlayerOne[Tail[SeedAndState]]]
                ]
            ]
        ]
    ]
)

Seed = StrToNum["221"]

Rec[Main][Pair[Seed][StartState]]