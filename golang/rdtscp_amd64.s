TEXT ·rdtscp(SB), $0-8
    RDTSCP
    SHLQ $32, DX
    ORQ  AX, DX
    MOVQ DX, ret+0(FP)
    RET