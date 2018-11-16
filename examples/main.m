PutStrLn["hello\x"]

PutStr[">> "]
RESPONSE = Input[_]

PutStrLn[
	If[Eq[RESPONSE]["hello\x"]]
		[":\rp"]
		[":\lp"]
]