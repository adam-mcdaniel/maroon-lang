PutStrln["hello!()[] @input"]

PutStr[">> "]
RESPONSE = Input[_]
PutStrln[
	If[Eq[RESPONSE]["hello!()[] @input"]]
		[":)"]
		[":("]
]
