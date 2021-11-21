.option norvc
.section .boot, "ax",@progbits
.global _start

_start:
    csrrwi  zero,0,0
    lui     sp,%hi(16384)
    ori     sp,sp,%lo(16384)
    j       __start_rust

.bss
