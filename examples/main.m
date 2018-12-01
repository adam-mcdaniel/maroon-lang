PutStrln["hello!"]

PutStr[">> "]
RESPONSE = Input[_]

PutStrln[
	If[Eq[RESPONSE]["hello!"]]
		[":)"]
		[":("]
]
