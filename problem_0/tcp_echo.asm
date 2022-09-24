; TCP Echo Service from RFC 862
; The solution to https://protohackers.com/problem/0
;
; Resources:
;  - Useful lookup table from: https://filippo.io/linux-syscall-table/

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
%define sin_family	2 ; AF_INET
%define sin_type	1 ; SOCK_STREAM
%define sin_addr	0 ; INADDR_ANY

section .bss
extern pthread_create
extern pthread_detach

section .data
thread_id: dq 0

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
    mov r9, rax ;r9 contains fd (return value)

    push dword sin_addr
    push word PORT
    push word sin_family

    ; bind(fd, *addr, addrlen)
    mov rax, sys_bind
    mov rdi, r9
    mov rsi, rsp
    mov rdx, 16 ; for sockaddr_in (IPv4)
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

    push r12 ; this still wrong, must be address(ed)

    mov rdi, thread_id ; &child_tid
    mov rsi, 0 ; NULL
    mov rdx, cl_loop ; cl_loop (func point)
    mov rcx, rsp ; &cl_fd
    
    call pthread_create

    mov rdi, thread_id
    call pthread_detach
    jmp srv_loop

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
