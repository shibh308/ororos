
LD_PATH=utils/link.ld
BOOT_PATH=utils/boot.s

BASENAME=ororos
OUT_DIR=out

A_PATH=rs/target/riscv32i-unknown-none-elf/debug/libororos_rs.a
EXE_PATH=$OUT_DIR/bin/$BASENAME
BIN_PATH=$OUT_DIR/$BASENAME.bin
HEX1_PATH=$OUT_DIR/hex/${BASENAME}_1.hex
HEX4_PATH=$OUT_DIR/hex/${BASENAME}_4.hex
DUMP_PATH=$OUT_DIR/dump/$BASENAME.dump
DUMP_L_PATH=$OUT_DIR/dump/${BASENAME}_alias.dump

cd rs/
/root/.cargo/bin/cargo +nightly build --target riscv32i-unknown-none-elf
cd ../

riscv64-unknown-elf-objdump -b elf32-littleriscv -M no-aliases -D $A_PATH | head -n 1000 > $DUMP_PATH
riscv64-unknown-elf-objdump -b elf32-littleriscv -D $A_PATH | head -n 1000 > $DUMP_L_PATH

riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -T $LD_PATH $BOOT_PATH $A_PATH -o $EXE_PATH -nostdlib

riscv64-unknown-elf-objcopy -O binary $EXE_PATH $BIN_PATH
od -An -tx1 -w1 -v $BIN_PATH > $HEX1_PATH
od -An -tx4 -w8 -v $BIN_PATH > $HEX4_PATH