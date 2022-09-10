EXEC_NAME=tcp_echo
SRC_NAME=tcp_echo
BUILD_FOLDER=bin

all: ${EXEC_NAME}
.PHONY: all ${EXEC_NAME} clean

${SRC_NAME}.o:
	mkdir -p ${BUILD_FOLDER}
	nasm -f elf64 -o ${BUILD_FOLDER}/${SRC_NAME}.o ${SRC_NAME}.asm

${EXEC_NAME}: ${SRC_NAME}.o
	ld ${BUILD_FOLDER}/${SRC_NAME}.o -o ${BUILD_FOLDER}/${EXEC_NAME}

clean:
	rm -rf ${BUILD_FOLDER}
