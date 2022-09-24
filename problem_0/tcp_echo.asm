; TCP Echo Service from RFC 862
; The solution to https://protohackers.com/problem/0
;
; Resources:
;  - Useful lookup table from: https://filippo.io/linux-syscall-table/

; sys/syscall.h
%define sys_read	0
%define sys_write	1
%define sys_close	3
%define sys_socket	41
%define sys_accept	43
%define sys_bind	49
%define sys_listen	50
%define sys_fork	57
%define sys_exit	60

; TCP configurations
%define MSGLEN		4096
%define PORT		0xefbe ; 48879
%define sin_family	2 ; AF_INET
%define sin_type	1 ; SOCK_STREAM
%define sin_addr	0 ; INADDR_ANY

section .text
exit:
    mov rdi, 0
    mov rax, sys_exit
    syscall

global main
main:
    ; fd = socket(AF_INET, SOCK_STREAM, 0);
    mov rax, sys_socket
    mov rdi, sin_family
    mov rsi, sin_type
    mov rdx, 0 ; default protocol
    syscall
    mov r8, rax ;r8 contains server fd

    push dword sin_addr
    push word PORT
    push word sin_family

    ; bind(fd, *addr, addrlen)
    mov rax, sys_bind
    mov rdi, r8
    mov rsi, rsp
    mov rdx, 16 ; for sockaddr_in (IPv4)
    syscall
    cmp rax, 0
    jl exit

    ; listen(sockfd, queue len)
    mov rax, sys_listen 
    mov rdi, r8
    mov rsi, 10
    syscall

; fork for every client that connects
srv_loop:
    ; accept(fd, addr(NULL), addrlen(NULL), flags)
    mov rax, sys_accept 
    mov rdi, r8
    mov rsi, 0
    mov rdx, 0
    syscall
    cmp rax, 0
    jl srv_loop
    mov r9, rax ; r9 fd to client

    ; fork()
    mov rax, sys_fork 
    syscall

    ; child process pid
    cmp rax, 0
    je echo

    ; close() client fd from main process
    mov rax, sys_close 
    mov rdi, r9
    syscall
    jmp srv_loop

; if (pid == 0) {
;   close(srv_fd);
;   echo(cl_fd);
;   close(cl_fd);
; }
echo:
    ; close() server fd from child process
    mov rax, sys_close 
    mov rdi, r8
    syscall

cl_loop:
    ; read(fd, buf, count)
    mov rax, sys_read
    mov rdi, r9
    mov rsi, msgbuffer
    mov rdx, MSGLEN
    syscall

    ; continue write from buffer if read bytes > 0
    cmp rax, 0
    jbe cl_exit

    ; write(fd, buf, count)
    mov rdx, rax ; rax contains result from prev read()
    mov rax, sys_write
    mov rdi, r9
    mov rsi, msgbuffer
    syscall
    jmp cl_loop

cl_exit:
    ; close() client fd 
    mov rax, sys_close 
    mov rdi, r9
    syscall

segment .bss
    msgbuffer: resb MSGLEN
