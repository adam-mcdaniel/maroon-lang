Log = Log_A.Log_B.(
    PipeFn[c.(PipeFn[a.(PipeStrln[Log_B])][PipeStr[Log_A]])][Log_B]
)

Info = Info_A.(Log["=( INFO )====> "][Info_A])
Debug = Debug_A.(Log["=| DEBUG |===> "][Debug_A])
Warning = Warning_A.(Log["={ WARNING }=> "][Warning_A])
Error = Error_A.(Log["=[ ERROR ]===> "][Error_A])

Good = True
Bad  = Bad_A.Bad_B.Bad_B