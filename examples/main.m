PutStrln["hello\x"]

PutStr[">> "]
RESPONSE = Input[_]

PutStrln[
	If[Eq[RESPONSE]["hello\x"]]
		[":\rp"]
		[":\lp"]
]