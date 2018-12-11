Import["logging"]
Import["fib"]
Import["num"]

Info["test"]
Debug["test"]
Warning["test"]
Error["test"]

print_fib[Succ[Succ[Succ[Succ[Succ[1]]]]]]


Debug[
    NumToStr[Succ[Succ[1]]]
]