@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ Low-level operations on Fp values
@
@ Each Fp value is stored as a word-aligned 12-word array
@
@ All functions work correctly with repeated arguments,
@ like e.g. fp_sum(x, x, x)
@
@ All functions should take constant time on ARM SC300
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.text

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_is_zero
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_is_zero
.syntax unified
.thumb
.thumb_func
.type fp_is_zero,	%function

fp_is_zero:
	ldr	r1, [r0,  #0]

	ldr	r2, [r0,  #4];	ldr	r3, [r0,  #8];	orr	r1, r2;	orr	r1, r3
	ldr	r2, [r0, #12];	ldr	r3, [r0, #16];	orr	r1, r2;	orr	r1, r3
	ldr	r2, [r0, #20];	ldr	r3, [r0, #24];	orr	r1, r2;	orr	r1, r3
	ldr	r2, [r0, #28];	ldr	r3, [r0, #32];	orr	r1, r2;	orr	r1, r3
	ldr	r2, [r0, #36];	ldr	r3, [r0, #40];	orr	r1, r2;	orr	r1, r3

	ldr	r2, [r0, #44];	orrs	r1, r2

	ite	eq
	moveq	r0, #1
	movne	r0, #0

	bx	lr

.size fp_is_zero, . - fp_is_zero

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_cpy: Copy
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_cpy
.syntax unified
.thumb
.thumb_func
.type fp_cpy,	%function

fp_cpy:
	ldr	r2, [r1,  #0]; ldr	r3, [r1,  #4]
	str	r2, [r0,  #0]; str	r3, [r0,  #4]

	ldr	r2, [r1,  #8]; ldr	r3, [r1, #12]
	str	r2, [r0,  #8]; str	r3, [r0, #12]

	ldr	r2, [r1, #16]; ldr	r3, [r1, #20]
	str	r2, [r0, #16]; str	r3, [r0, #20]

	ldr	r2, [r1, #24]; ldr	r3, [r1, #28]
	str	r2, [r0, #24]; str	r3, [r0, #28]

	ldr	r2, [r1, #32]; ldr	r3, [r1, #36]
	str	r2, [r0, #32]; str	r3, [r0, #36]

	ldr	r2, [r1, #40]; ldr	r3, [r1, #44]
	str	r2, [r0, #40]; str	r3, [r0, #44]

	bx	lr

.size fp_cpy, . - fp_cpy

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_eq: Check two Fp values for equality
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_eq
.syntax unified
.thumb
.thumb_func
.type fp_eq,	%function

fp_eq:
	sub	sp, #4

	ldr	r2, [r0,  #0]
	ldr	r3, [r1,  #0]
	str	r4, [sp]
	sub	r2, r3

	ldr	r3, [r0,  #4];	ldr	r4, [r1,  #4];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0,  #8];	ldr	r4, [r1,  #8];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #12];	ldr	r4, [r1, #12];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #16];	ldr	r4, [r1, #16];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #20];	ldr	r4, [r1, #20];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #24];	ldr	r4, [r1, #24];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #28];	ldr	r4, [r1, #28];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #32];	ldr	r4, [r1, #32];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #36];	ldr	r4, [r1, #36];	sub	r3, r4;	orr	r2, r3 
	ldr	r3, [r0, #40];	ldr	r4, [r1, #40];	sub	r3, r4;	orr	r2, r3 

	ldr	r4, [sp]

	ldr	r0, [r0, #44];	ldr	r1, [r1, #44];	sub	r0, r1;	orrs	r0, r2 

	ite	eq
	moveq	r0, #1
	movne	r0, #0

	add	sp, #4

	bx	lr

.size fp_eq, . - fp_eq

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_neg: Negate an Fp value
@
@ x = (y != 0) ? (p - y) : 0
@
@  x  = (y == 0) ? p : y
@  x  = -x
@  x += p
@
@  Note: -x == ~x + 1
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_neg
.syntax unified
.thumb
.thumb_func
.type fp_neg,	%function

fp_neg:
	push	{ r4-r12, lr }
	ldm	r1, { r1-r12 }

	@ lr = (y == 0)

	orr	lr, r1, r2
	orr	lr, r3
	orr	lr, r4
	orr	lr, r5
	orr	lr, r6
	orr	lr, r7
	orr	lr, r8
	orr	lr, r9
	orr	lr, r10
	orr	lr, r11
	orrs	lr, r12	@ Z = (lr == 0) ? 1 : 0

	@ x = Z ? p : y

	itttt	eq
	moveq	r1, #0x00000001
	moveq	r2, #0xc000
	movteq	r2, #0x8508
	moveq	r3, #0x30000000

	itttt	eq
	moveq	r4, #0x5D44
	movteq	r4, #0x170B
	moveq	r5, #0x4800
	movteq	r5, #0xBA09

	itttt	eq
	moveq	r6, #0x622F
	movteq	r6, #0x1EF3
	moveq	r7, #0x138F
	movteq	r7, #0x00F5

	itttt	eq
	moveq	r8, #0xD9F3
	movteq	r8, #0x1A22
	moveq	r9, #0x493B
	movteq	r9, #0x6CA1

	itttt	eq
	moveq	r10, #0x05C0
	movteq	r10, #0xC63B
	moveq	r11, #0x10EA
	movteq	r11, #0x17C5

	itt	eq
	moveq	r12, #0x3A46
	movteq	r12, #0x01AE

	@ x = ~x

	mvn	 r1,  r1
	mvn	 r2,  r2
	mvn	 r3,  r3
	mvn	 r4,  r4
	mvn	 r5,  r5
	mvn	 r6,  r6
	mvn	 r7,  r7
	mvn	 r8,  r8
	mvn	 r9,  r9
	mvn	r10, r10
	mvn	r11, r11
	mvn	r12, r12

	@ x += 1

	adds	 r1, #1
	adcs	 r2, #0
	adcs	 r3, #0
	adcs	 r4, #0
	adcs	 r5, #0
	adcs	 r6, #0
	adcs	 r7, #0
	adcs	 r8, #0
	adcs	 r9, #0
	adcs	r10, #0
	adcs	r11, #0
	adc	r12, #0

	@ x += p

	adds	r1, #0x00000001

	mov	lr, #0xc000;	movt	lr, #0x8508;	adcs	 r2, lr

	adcs	r3, #0x30000000

	mov	lr, #0x5D44;	movt	lr, #0x170B;	adcs	 r4, lr
	mov	lr, #0x4800;	movt	lr, #0xBA09;	adcs	 r5, lr
	mov	lr, #0x622F;	movt	lr, #0x1EF3;	adcs	 r6, lr
	mov	lr, #0x138F;	movt	lr, #0x00F5;	adcs	 r7, lr
	mov	lr, #0xD9F3;	movt	lr, #0x1A22;	adcs	 r8, lr
	mov	lr, #0x493B;	movt	lr, #0x6CA1;	adcs	 r9, lr
	mov	lr, #0x05C0;	movt	lr, #0xC63B;	adcs	r10, lr
	mov	lr, #0x10EA;	movt	lr, #0x17C5;	adcs	r11, lr
	mov	lr, #0x3A46;	movt	lr, #0x01AE;	adcs	r12, lr

	stm	r0, { r1-r12 }
	pop	    { r4-r12, pc }

.size fp_neg, . - fp_neg

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_diff: x = y - z (mod p)
@
@  x = y - z
@  store x
@
@  C = (x < 0); x += p
@  if (C)	/* carry, because x was < 0 */
@    store x
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_diff
.syntax unified
.thumb
.thumb_func
.type fp_diff,	%function

fp_diff:
	push	{ r4-r12, lr }

	@ x = y - z

	ldr	r14, [r1,  #0]
	ldr	r12, [r1,  #4]
	ldr	r11, [r1,  #8]
	ldr	r10, [r1, #12]
	ldr	 r9, [r1, #16]

	ldr	 r8, [r2,  #0]
	ldr	 r7, [r2,  #4]
	ldr	 r6, [r2,  #8]
	ldr	 r5, [r2, #12]
	ldr	 r4, [r2, #16]

	subs	r14,  r8
	sbcs	r12,  r7
	sbcs	r11,  r6
	sbcs	r10,  r5
	sbcs	 r9,  r4

	ldr	 r8, [r1, #20]
	ldr	 r7, [r1, #24]
	ldr	 r6, [r1, #28]

	ldr	 r5, [r2, #20]
	ldr	 r4, [r2, #24]
	ldr	 r3, [r2, #28]

	str	r14, [r0,  #0]	@ store early (0 cycles)

	sbcs	 r8,  r5
	sbcs	 r7,  r4
	sbcs	 r6,  r3

	ldr	 r5, [r1, #32]
	ldr	 r4, [r1, #36]

	ldr	 r3, [r2, #32]
	ldr	r14, [r2, #36]	@ use r14 as temporary

	str	r12, [r0,  #4]	@ store early (0 cycles)

	sbcs	 r5,  r3
	sbcs	 r4, r14

	ldr	 r3, [r1, #40]
	ldr	 r1, [r1, #44]

	ldr	r14, [r2, #40]
	ldr	 r2, [r2, #44]

	str	r11, [r0,  #8]	@ store early (0 cycles)

	sbcs	 r3, r14
	sbcs	 r2,  r1,  r2

	ldr	r14, [r0,  #0]	@ restore r14

	str	r10, [r0, #12]
	str	 r9, [r0, #16]
	str	 r8, [r0, #20]
	str	 r7, [r0, #24]
	str	 r6, [r0, #28]
	str	 r5, [r0, #32]
	str	 r4, [r0, #36]
	str	 r3, [r0, #40]
	str	 r2, [r0, #44]

	@ x += p, using r1 as temporary, x in { r14, r12-r2 }

	adds	r14, #0x00000001

	mov	r1, #0xc000;	movt	r1, #0x8508;	adcs	r12, r1

	adcs	r11, #0x30000000

	mov	r1, #0x5D44;	movt	r1, #0x170B;	adcs	r10, r1
	mov	r1, #0x4800;	movt	r1, #0xBA09;	adcs	 r9, r1
	mov	r1, #0x622F;	movt	r1, #0x1EF3;	adcs	 r8, r1
	mov	r1, #0x138F;	movt	r1, #0x00F5;	adcs	 r7, r1
	mov	r1, #0xD9F3;	movt	r1, #0x1A22;	adcs	 r6, r1
	mov	r1, #0x493B;	movt	r1, #0x6CA1;	adcs	 r5, r1
	mov	r1, #0x05C0;	movt	r1, #0xC63B;	adcs	 r4, r1
	mov	r1, #0x10EA;	movt	r1, #0x17C5;	adcs	 r3, r1
	mov	r1, #0x3A46;	movt	r1, #0x01AE;	adcs	 r2, r1

	itttt	cs	@ carry set => x was negative => we need to store x
	strcs	r14, [r0,  #0]
	strcs	r12, [r0,  #4]
	strcs	r11, [r0,  #8]
	strcs	r10, [r0, #12]
	itttt	cs
	strcs	 r9, [r0, #16]
	strcs	 r8, [r0, #20]
	strcs	 r7, [r0, #24]
	strcs	 r6, [r0, #28]
	itttt	cs
	strcs	 r5, [r0, #32]
	strcs	 r4, [r0, #36]
	strcs	 r3, [r0, #40]
	strcs	 r2, [r0, #44]

	pop	{ r4-r12, pc }

.size fp_diff, . - fp_diff

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_sum: x = y + z (mod p)
@
@  x = y + z
@  store x
@
@  C = (x >= p); x -= p
@  if (C)
@    store x
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_sum
.syntax unified
.thumb
.thumb_func
.type fp_sum,	%function

fp_sum:
	push	    { r4-r12, lr }

	@ x = y + z

	ldr	r14, [r1,  #0]
	ldr	r12, [r1,  #4]
	ldr	r11, [r1,  #8]
	ldr	r10, [r1, #12]
	ldr	 r9, [r1, #16]

	ldr	 r8, [r2,  #0]
	ldr	 r7, [r2,  #4]
	ldr	 r6, [r2,  #8]
	ldr	 r5, [r2, #12]
	ldr	 r4, [r2, #16]

	adds	r14,  r8
	adcs	r12,  r7
	adcs	r11,  r6
	adcs	r10,  r5
	adcs	 r9,  r4

	ldr	 r8, [r1, #20]
	ldr	 r7, [r1, #24]
	ldr	 r6, [r1, #28]

	ldr	 r5, [r2, #20]
	ldr	 r4, [r2, #24]
	ldr	 r3, [r2, #28]

	str	r14, [r0,  #0]	@ store early (0 cycles)

	adcs	 r8,  r5
	adcs	 r7,  r4
	adcs	 r6,  r3

	ldr	 r5, [r1, #32]
	ldr	 r4, [r1, #36]

	ldr	 r3, [r2, #32]
	ldr	r14, [r2, #36]	@ use r14 as temporary

	str	r12, [r0,  #4]	@ store early (0 cycles)

	adcs	 r5,  r3
	adcs	 r4, r14

	ldr	 r3, [r1, #40]
	ldr	 r1, [r1, #44]

	ldr	r14, [r2, #40]
	ldr	 r2, [r2, #44]

	str	r11, [r0,  #8]	@ store early (0 cycles)

	adcs	 r3, r14
	adcs	 r2,  r1,  r2

	ldr	r14, [r0,  #0]	@ restore r14

	str	r10, [r0, #12]
	str	 r9, [r0, #16]
	str	 r8, [r0, #20]
	str	 r7, [r0, #24]
	str	 r6, [r0, #28]
	str	 r5, [r0, #32]
	str	 r4, [r0, #36]
	str	 r3, [r0, #40]
	str	 r2, [r0, #44]

	@ x -= p, using r1 as temporary, x in { r14, r12-r2 }

	subs	r14, #0x00000001

	mov	r1, #0xc000;	movt	r1, #0x8508;	sbcs	r12, r1

	sbcs	r11, #0x30000000

	mov	r1, #0x5D44;	movt	r1, #0x170B;	sbcs	r10, r1
	mov	r1, #0x4800;	movt	r1, #0xBA09;	sbcs	 r9, r1
	mov	r1, #0x622F;	movt	r1, #0x1EF3;	sbcs	 r8, r1
	mov	r1, #0x138F;	movt	r1, #0x00F5;	sbcs	 r7, r1
	mov	r1, #0xD9F3;	movt	r1, #0x1A22;	sbcs	 r6, r1
	mov	r1, #0x493B;	movt	r1, #0x6CA1;	sbcs	 r5, r1
	mov	r1, #0x05C0;	movt	r1, #0xC63B;	sbcs	 r4, r1
	mov	r1, #0x10EA;	movt	r1, #0x17C5;	sbcs	 r3, r1
	mov	r1, #0x3A46;	movt	r1, #0x01AE;	sbcs	 r2, r1

	itttt	cs	@ carry set == no borrow => x was >= p => we need to store x
	strcs	r14, [r0,  #0]
	strcs	r12, [r0,  #4]
	strcs	r11, [r0,  #8]
	strcs	r10, [r0, #12]
	itttt	cs
	strcs	 r9, [r0, #16]
	strcs	 r8, [r0, #20]
	strcs	 r7, [r0, #24]
	strcs	 r6, [r0, #28]
	itttt	cs
	strcs	 r5, [r0, #32]
	strcs	 r4, [r0, #36]
	strcs	 r3, [r0, #40]
	strcs	 r2, [r0, #44]

	pop	    { r4-r12, pc }

.size fp_sum, . - fp_sum

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_cset: x = c ? y : x
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_cset
.syntax unified
.thumb
.thumb_func
.type fp_cset,	%function

fp_cset:
	movs	r2, r2

	ldr	r2, [r1,  #0]
	ldr	r3, [r1,  #4]
	itt	ne
	strne	r2, [r0,  #0]
	strne	r3, [r0,  #4]

	ldr	r2, [r1,  #8]
	ldr	r3, [r1, #12]
	itt	ne
	strne	r2, [r0,  #8]
	strne	r3, [r0, #12]

	ldr	r2, [r1, #16]
	ldr	r3, [r1, #20]
	itt	ne
	strne	r2, [r0, #16]
	strne	r3, [r0, #20]

	ldr	r2, [r1, #24]
	ldr	r3, [r1, #28]
	itt	ne
	strne	r2, [r0, #24]
	strne	r3, [r0, #28]

	ldr	r2, [r1, #32]
	ldr	r3, [r1, #36]
	itt	ne
	strne	r2, [r0, #32]
	strne	r3, [r0, #36]

	ldr	r2, [r1, #40]
	ldr	r3, [r1, #44]
	itt	ne
	strne	r2, [r0, #40]
	strne	r3, [r0, #44]

	bx	lr

.size fp_cset, . - fp_cset

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_to_bytes
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_to_bytes
.syntax unified
.thumb
.thumb_func
.type fp_to_bytes,	%function

fp_to_bytes:
				ldr	r2, [r1, #44]
	strb	r2, [r0,  #3];	lsrs	r2, #8
	strb	r2, [r0,  #2];	lsrs	r2, #8
	strb	r2, [r0,  #1];	lsrs	r2, #8
	strb	r2, [r0,  #0];	ldr	r2, [r1, #40]

	strb	r2, [r0,  #7];	lsrs	r2, #8
	strb	r2, [r0,  #6];	lsrs	r2, #8
	strb	r2, [r0,  #5];	lsrs	r2, #8
	strb	r2, [r0,  #4];	ldr	r2, [r1, #36]

	strb	r2, [r0, #11];	lsrs	r2, #8
	strb	r2, [r0, #10];	lsrs	r2, #8
	strb	r2, [r0,  #9];	lsrs	r2, #8
	strb	r2, [r0,  #8];	ldr	r2, [r1, #32]

	strb	r2, [r0, #15];	lsrs	r2, #8
	strb	r2, [r0, #14];	lsrs	r2, #8
	strb	r2, [r0, #13];	lsrs	r2, #8
	strb	r2, [r0, #12];	ldr	r2, [r1, #28]

	strb	r2, [r0, #19];	lsrs	r2, #8
	strb	r2, [r0, #18];	lsrs	r2, #8
	strb	r2, [r0, #17];	lsrs	r2, #8
	strb	r2, [r0, #16];	ldr	r2, [r1, #24]

	strb	r2, [r0, #23];	lsrs	r2, #8
	strb	r2, [r0, #22];	lsrs	r2, #8
	strb	r2, [r0, #21];	lsrs	r2, #8
	strb	r2, [r0, #20];	ldr	r2, [r1, #20]

	strb	r2, [r0, #27];	lsrs	r2, #8
	strb	r2, [r0, #26];	lsrs	r2, #8
	strb	r2, [r0, #25];	lsrs	r2, #8
	strb	r2, [r0, #24];	ldr	r2, [r1, #16]

	strb	r2, [r0, #31];	lsrs	r2, #8
	strb	r2, [r0, #30];	lsrs	r2, #8
	strb	r2, [r0, #29];	lsrs	r2, #8
	strb	r2, [r0, #28];	ldr	r2, [r1, #12]

	strb	r2, [r0, #35];	lsrs	r2, #8
	strb	r2, [r0, #34];	lsrs	r2, #8
	strb	r2, [r0, #33];	lsrs	r2, #8
	strb	r2, [r0, #32];	ldr	r2, [r1,  #8]

	strb	r2, [r0, #39];	lsrs	r2, #8
	strb	r2, [r0, #38];	lsrs	r2, #8
	strb	r2, [r0, #37];	lsrs	r2, #8
	strb	r2, [r0, #36];	ldr	r2, [r1,  #4]

	strb	r2, [r0, #43];	lsrs	r2, #8
	strb	r2, [r0, #42];	lsrs	r2, #8
	strb	r2, [r0, #41];	lsrs	r2, #8
	strb	r2, [r0, #40];	ldr	r2, [r1,  #0]

	strb	r2, [r0, #47];	lsrs	r2, #8
	strb	r2, [r0, #46];	lsrs	r2, #8
	strb	r2, [r0, #45];	lsrs	r2, #8
	strb	r2, [r0, #44]

	bx	lr

.size fp_to_bytes, . - fp_to_bytes

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fp_from_bytes
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fp_from_bytes
.syntax unified
.thumb
.thumb_func
.type fp_from_bytes,	%function

fp_from_bytes:
	ldrb	r2, [r1,  #0];	strb	r2, [r0, #47]
	ldrb	r2, [r1,  #1];	strb	r2, [r0, #46]
	ldrb	r2, [r1,  #2];	strb	r2, [r0, #45]
	ldrb	r2, [r1,  #3];	strb	r2, [r0, #44]

	ldrb	r2, [r1,  #4];	strb	r2, [r0, #43]
	ldrb	r2, [r1,  #5];	strb	r2, [r0, #42]
	ldrb	r2, [r1,  #6];	strb	r2, [r0, #41]
	ldrb	r2, [r1,  #7];	strb	r2, [r0, #40]

	ldrb	r2, [r1,  #8];	strb	r2, [r0, #39]
	ldrb	r2, [r1,  #9];	strb	r2, [r0, #38]
	ldrb	r2, [r1, #10];	strb	r2, [r0, #37]
	ldrb	r2, [r1, #11];	strb	r2, [r0, #36]

	ldrb	r2, [r1, #12];	strb	r2, [r0, #35]
	ldrb	r2, [r1, #13];	strb	r2, [r0, #34]
	ldrb	r2, [r1, #14];	strb	r2, [r0, #33]
	ldrb	r2, [r1, #15];	strb	r2, [r0, #32]

	ldrb	r2, [r1, #16];	strb	r2, [r0, #31]
	ldrb	r2, [r1, #17];	strb	r2, [r0, #30]
	ldrb	r2, [r1, #18];	strb	r2, [r0, #29]
	ldrb	r2, [r1, #19];	strb	r2, [r0, #28]

	ldrb	r2, [r1, #20];	strb	r2, [r0, #27]
	ldrb	r2, [r1, #21];	strb	r2, [r0, #26]
	ldrb	r2, [r1, #22];	strb	r2, [r0, #25]
	ldrb	r2, [r1, #23];	strb	r2, [r0, #24]

	ldrb	r2, [r1, #24];	strb	r2, [r0, #23]
	ldrb	r2, [r1, #25];	strb	r2, [r0, #22]
	ldrb	r2, [r1, #26];	strb	r2, [r0, #21]
	ldrb	r2, [r1, #27];	strb	r2, [r0, #20]

	ldrb	r2, [r1, #28];	strb	r2, [r0, #19]
	ldrb	r2, [r1, #29];	strb	r2, [r0, #18]
	ldrb	r2, [r1, #30];	strb	r2, [r0, #17]
	ldrb	r2, [r1, #31];	strb	r2, [r0, #16]

	ldrb	r2, [r1, #32];	strb	r2, [r0, #15]
	ldrb	r2, [r1, #33];	strb	r2, [r0, #14]
	ldrb	r2, [r1, #34];	strb	r2, [r0, #13]
	ldrb	r2, [r1, #35];	strb	r2, [r0, #12]

	ldrb	r2, [r1, #36];	strb	r2, [r0, #11]
	ldrb	r2, [r1, #37];	strb	r2, [r0, #10]
	ldrb	r2, [r1, #38];	strb	r2, [r0,  #9]
	ldrb	r2, [r1, #39];	strb	r2, [r0,  #8]

	ldrb	r2, [r1, #40];	strb	r2, [r0,  #7]
	ldrb	r2, [r1, #41];	strb	r2, [r0,  #6]
	ldrb	r2, [r1, #42];	strb	r2, [r0,  #5]
	ldrb	r2, [r1, #43];	strb	r2, [r0,  #4]

	ldrb	r2, [r1, #44];	strb	r2, [r0,  #3]
	ldrb	r2, [r1, #45];	strb	r2, [r0,  #2]
	ldrb	r2, [r1, #46];	strb	r2, [r0,  #1]
	ldrb	r2, [r1, #47];	strb	r2, [r0,  #0]

	@ Subtract modulus

	ldr	r2, [r0,  #0]
	mov	r3, #0x00000001
	subs	r2, r3

	ldr	r2, [r0,  #4]
	mov	r3, #0xC000
	movt	r3, #0x8508
	sbcs	r2, r3

	ldr	r2, [r0,  #8]
	mov	r3, #0x30000000
	sbcs	r2, r3

	ldr	r2, [r0, #12]
	mov	r3, #0x5D44
	movt	r3, #0x170B
	sbcs	r2, r3

	ldr	r2, [r0, #16]
	mov	r3, #0x4800
	movt	r3, #0xBA09
	sbcs	r2, r3

	ldr	r2, [r0, #20]
	mov	r3, #0x622F
	movt	r3, #0x1EF3
	sbcs	r2, r3

	ldr	r2, [r0, #24]
	mov	r3, #0x138F
	movt	r3, #0x00F5
	sbcs	r2, r3

	ldr	r2, [r0, #28]
	mov	r3, #0xD9F3
	movt	r3, #0x1A22
	sbcs	r2, r3

	ldr	r2, [r0, #32]
	mov	r3, #0x493B
	movt	r3, #0x6CA1
	sbcs	r2, r3

	ldr	r2, [r0, #36]
	mov	r3, #0x05C0
	movt	r3, #0xC63B
	sbcs	r2, r3

	ldr	r2, [r0, #40]
	mov	r3, #0x10EA
	movt	r3, #0x17C5
	sbcs	r2, r3

	ldr	r2, [r0, #44]
	mov	r3, #0x3A46
	movt	r3, #0x01AE
	sbcs	r2, r3

	@ Malformed input (x>=m) => no borrow

	mov	r2, #0

	itttt	cs	@ carry set == no borrow => x was >= p => we need to zero x
	strcs	r2, [r0,  #0]
	strcs	r2, [r0,  #4]
	strcs	r2, [r0,  #8]
	strcs	r2, [r0, #12]

	itttt	cs
	strcs	r2, [r0, #16]
	strcs	r2, [r0, #20]
	strcs	r2, [r0, #24]
	strcs	r2, [r0, #28]

	itttt	cs
	strcs	r2, [r0, #32]
	strcs	r2, [r0, #36]
	strcs	r2, [r0, #40]
	strcs	r2, [r0, #44]

	bx	lr

.size fp_from_bytes, . - fp_from_bytes
