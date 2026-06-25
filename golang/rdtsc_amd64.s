TEXT ·rdtsc(SB), $0-8
    RDTSC
    SHLQ $32, DX
    ORQ  AX, DX
    MOVQ DX, ret+0(FP)
    RET
