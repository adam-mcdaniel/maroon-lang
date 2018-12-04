PutStrln["hello!"]

PutStr[">> "]
RESPONSE = Input[_]
Putln[RESPONSE]
Putln["hello!()[]"]
PutStrln[
	If[Eq[RESPONSE]["hello!()[]"]]
		[":)"]
		[":("]
]
