Log = Log_A.Log_B.(
    PipeFn[a.(PipeStrln[Log_B])][PipeStr[Log_A]]
)

Info = Info_A.(Log["=( INFO )====> "][Info_A])
Debug = Debug_A.(Log["=| DEBUG |===> "][Debug_A])
Warning = Warning_A.(Log["={ WARNING }=> "][Warning_A])
Error = Error_A.(Log["=[ ERROR ]===> "][Error_A])

Info["hey jude"]
Debug["hey jude"]
Warning["hey jude"]
Error["hey jude"]