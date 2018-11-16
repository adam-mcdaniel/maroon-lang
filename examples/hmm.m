// Main = State.(Input[PipeStr[State]])

// "begin" Main @rec


Main = State.(
    USER.(
        If[Eq[State]["Run"]]
            [
                If[Eq[USER]["okay"]]
                    ["Come on"]
                    ["Run"]
            ]
            [
                If[Eq[State]["Come on"]]
                    [
                        If[Eq[USER]["im trying"]]
                            ["Youre almost there"]
                            ["Run"]
                    ]
                    [
                        If[Eq[State]["Youre almost there"]]
                            ["Youre almost there"]
                            ["Run"]
                    ]
            ]
    )[Input[_]]
)

starting_state = "Run"

PutStrln[starting_state]
Rec[(s.(PipeStrln[Main[s]]))][starting_state]