.option norvc
.section .boot, "ax",@progbits
.global _start

_start:
    csrrwi  zero,0,0
    lui     sp,%hi(4096)
    ori     sp,sp,%lo(4096)
    j       __start_rust

.bss
