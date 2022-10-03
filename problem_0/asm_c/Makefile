ASM_EXEC_NAME=tcp_echo_asm
CLANG_EXEC_NAME=tcp_echo_clang
SRC_NAME=tcp_echo
BUILD_FOLDER=bin

all: asm clang
.PHONY: all asm clean

${SRC_NAME}.o:
	mkdir -p ${BUILD_FOLDER}
	nasm -f elf64 -g -o ${BUILD_FOLDER}/${SRC_NAME}.o ${SRC_NAME}.asm

asm: ${SRC_NAME}.o
	ld ${BUILD_FOLDER}/${SRC_NAME}.o -o ${BUILD_FOLDER}/${ASM_EXEC_NAME}

clang:
	gcc ${SRC_NAME}.c -o ${BUILD_FOLDER}/${CLANG_EXEC_NAME}

clean:
	rm -rf ${BUILD_FOLDER}
