sandboxed::Sandboxed::foo:
	push r15
	push r14
	push r13
	push r12
	push rbx
	sub rsp, 48
	mov rbx, rsi
	mov r14, rdi
	call qword ptr [rip + mpk::sandbox::Sandbox::init@GOTPCREL]
	cmp eax, 1
	jne .LBB310_6
	mov r15, qword ptr [rip + mpk::sandbox::RETURNSTACK@GOTPCREL]
	cmp qword ptr [r15], 0
	jne .LBB310_7
	mov r13, qword ptr [r14 + 16]
	lea r14, [r13 + 8388592]
	mov qword ptr [r13 + 8388592], rbx
	xor ecx, ecx
	rdpkru
	mov r12d, eax
	mov edi, eax
	mov esi, 11
	mov edx, 2
	call qword ptr [rip + mpk::sandbox::pkru_set@GOTPCREL]
	mov rdi, r14
	xor ecx, ecx
	xor edx, edx
	mov qword ptr [rip + mpk::sandbox::RETURNSTACK], rsp
	mov rsp, rdi
	wrpkru
	call mpk::sandbox::_sandbox_call
	mov rax, r12
	xor rcx, rcx
	xor rdx, rdx
	wrpkru
	mov rsp, qword ptr [rip + mpk::sandbox::RETURNSTACK]
	mov qword ptr [r15], 0
	mov rax, qword ptr [r13 + 8388592]
	jmp .LBB310_3
.LBB310_6:
	mov rdi, rbx
	call qword ptr [rip + foo@GOTPCREL]
.LBB310_3:
	test rax, rax
	je .LBB310_8
	test al, 3
	jne .LBB310_9
	add rsp, 48
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	ret
