BASENAME=$(basename ${1%.*})
if [ -z "$BASENAME" ]; then
    echo "invalid argument"
    exit
fi
echo $BASENAME

LD_PATH=utils/link.ld

OUT_DIR=out
O_PATH=$OUT_DIR/$BASENAME.o
EXE_PATH=$OUT_DIR/bin/$BASENAME
BIN_PATH=$OUT_DIR/$BASENAME.bin
HEX_PATH=$OUT_DIR/hex/$BASENAME.hex
DUMP_PATH=$OUT_DIR/dump/$BASENAME.dump

riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -c -o $O_PATH $1
riscv64-unknown-elf-ld -b elf32-littleriscv $O_PATH -T $LD_PATH -o $EXE_PATH
riscv64-unknown-elf-objcopy -O binary $OUT_PATH/$BASENAME $BIN_PATH
od -An -tx1 -w1 -v $BIN_PATH > $HEX_PATH
riscv64-unknown-elf-objdump -b elf32-littleriscv -M no-aliases --section=.text --section=.text.startup --section=.text.init --section=.data -D $EXE_PATH > $DUMP_PATH
rm $O_PATH $BIN_PATH