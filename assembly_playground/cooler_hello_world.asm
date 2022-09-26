section .data
msg db "hello world !", 0Ah

section .text
global _start

_start:
	mov ebx, msg ; moves the address of msg to ebx
	mov eax, ebx ; move contents of ebx to eax

next_char:
	cmp byte[eax], 0 ; cmp the data in eax and string termination character or null
	jz	_print ; jump is zero
	inc eax ; increament eacx
	jmp next_char ;loop

_print:
	sub eax, ebx
	mov edx, eax,
	mov ecx, msg
	mov ebx, 1
	mov eax, 4
	int 0x80
	
	mov ebx, 0
	mov eax, 1
	int 0x80
