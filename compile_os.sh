PACKAGE_NAME=ctest
WORK_DIR=/opt/riscv/riscv_chisel_cpu
RESULT_DIR=$WORK_DIR/results
READ_DIR=$WORK_DIR/src/templates
WRITE_DIR=$WORK_DIR/src/main/scala/auto_generate/$PACKAGE_NAME

LOAD_PATH=/opt/riscv/ororos/out/hex/ororos_1.hex
LD_PATH=src/c/link.ld

mkdir -p $WRITE_DIR
cd $WORK_DIR

sed -e "s/{package}/$PACKAGE_NAME/" -e "s/{exit}/(inst === UNIMP)/" $READ_DIR/Core.scala > $WRITE_DIR/Core.scala
sed -e "s/{package}/$PACKAGE_NAME/" $READ_DIR/Top.scala > $WRITE_DIR/Top.scala
sed -e "s/{package}/$PACKAGE_NAME/" $READ_DIR/Serial.scala > $WRITE_DIR/Serial.scala
sed -e "s/{package}/$PACKAGE_NAME/" -e "s%{load_path}%$LOAD_PATH%" $READ_DIR/Memory.scala > $WRITE_DIR/Memory.scala

sbt "testOnly ctest.HexTest"