; TCP Echo Service from RFC 862
; The solution to https://protohackers.com/problem/0
;
; Resources:
;  - Useful lookup table from: https://filippo.io/linux-syscall-table/

global _start

; sys/syscall.h
%define sys_read	0
%define sys_write	1
%define sys_socket	41
%define sys_accept	43
%define sys_bind	49
%define sys_listen	50
%define sys_fork	57
%define sys_exit	60

; unistd.h
%define STDIN		0
%define STDOUT		1
%define STDERR		2

; TCP configurations
%define MSGLEN		512
%define PORT		0xefbe

section .text

exit:
    mov rdi, 0
    mov rax, sys_exit
    syscall

_start:
    ; fd = socket(AF_INET, SOCK_STREAM, 0);
    mov rax, sys_socket
    mov rdi, 2
    mov rsi, 1
    mov rdx, 0
    syscall
    mov r9,rax ;r9 contains fd (return value)

    push dword 0      ; INADDR_ANY
    push word PORT
    push word 2       ; AF_INET

    ; bind(fd, *addr, addrlen)
    mov rax, sys_bind
    mov rdi, r9
    mov rsi, rsp
    mov rdx, 16
    syscall
    cmp rax, 0
    jl exit

    ; listen(sockfd, queue len)
    mov rax, sys_listen 
    mov rdi, r9
    mov rsi, 10
    syscall

; fork a thread for every client that connects
srv_loop:
    ; accept(fd, addr(NULL), addrlen(NULL), flags)
    mov rax, sys_accept 
    mov rdi, r9
    mov rsi, 0
    mov rdx, 0
    syscall
    cmp rax, 0
    jl srv_loop
    mov r12, rax ; r12 fd to client

    ; fork()
    mov rax, sys_fork 
    syscall
    cmp rax, 0 ; child return is 0
    jne srv_loop

cl_loop:
    ; read(fd, buf, count)
    mov rax, sys_read
    mov rdi, r12
    mov rsi, msgbuffer
    mov rdx, MSGLEN
    syscall

    ; write(fd, buf, count)
    mov rax, sys_write
    mov rdi, r12
    mov rsi, msgbuffer
    mov rdx, MSGLEN
    syscall
    jmp cl_loop

segment .bss
    msgbuffer: resb MSGLEN