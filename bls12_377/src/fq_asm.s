@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ Low-level operations on Fq values
@
@ Each Fq value is stored as a word-aligned 8-word array
@
@ All functions work correctly with repeated arguments,
@ like e.g. fq_sum(x, x, x)
@
@ All functions should take constant time on ARM SC300
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.text

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_is_zero
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_is_zero
.syntax unified
.thumb
.thumb_func
.type fq_is_zero,	%function

fq_is_zero:
	ldr	r1, [r0,  #0]

	ldr	r2, [r0,  #4];	ldr	r3, [r0,  #8];	orr	r1, r2;	orr	r1, r3
	ldr	r2, [r0, #12];	ldr	r3, [r0, #16];	orr	r1, r2;	orr	r1, r3
	ldr	r2, [r0, #20];	ldr	r3, [r0, #24];	orr	r1, r2;	orr	r1, r3

	ldr	r2, [r0, #20];	orrs	r1, r2

	ite	eq
	moveq	r0, #1
	movne	r0, #0

	bx	lr

.size fq_is_zero, . - fq_is_zero

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_cpy: Copy
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_cpy
.syntax unified
.thumb
.thumb_func
.type fq_cpy,	%function

fq_cpy:
	ldr	r2, [r1,  #0]; ldr	r3, [r1,  #4]
	str	r2, [r0,  #0]; str	r3, [r0,  #4]

	ldr	r2, [r1,  #8]; ldr	r3, [r1, #12]
	str	r2, [r0,  #8]; str	r3, [r0, #12]

	ldr	r2, [r1, #16]; ldr	r3, [r1, #20]
	str	r2, [r0, #16]; str	r3, [r0, #20]

	ldr	r2, [r1, #24]; ldr	r3, [r1, #28]
	str	r2, [r0, #24]; str	r3, [r0, #28]

	bx	lr

.size fq_cpy, . - fq_cpy

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_eq: Check two Fq values for equality
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_eq
.syntax unified
.thumb
.thumb_func
.type fq_eq,	%function

fq_eq:
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

	ldr	r4, [sp]

	ldr	r0, [r0, #28];	ldr	r1, [r1, #28];	sub	r0, r1;	orrs	r0, r2 

	ite	eq
	moveq	r0, #1
	movne	r0, #0

	add	sp, #4

	bx	lr

.size fq_eq, . - fq_eq

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_neg: Negate an Fq value
@
@ x = (y != 0) ? (q - y) : 0
@
@  x  = (y == 0) ? q : y
@  x  = -x
@  x += q
@
@  Note: -x == ~x + 1
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_neg
.syntax unified
.thumb
.thumb_func
.type fq_neg,	%function

fq_neg:
	push	{ r4-r8, lr }
	ldm	r1, { r1-r8 }

	@ lr = (y == 0)

	orr	lr, r1, r2
	orr	lr, r3
	orr	lr, r4
	orr	lr, r5
	orr	lr, r6
	orr	lr, r7
	orrs	lr, r8	@ Z = (lr == 0) ? 1 : 0

	@ x = Z ? q : y

	itttt	eq
	moveq	r1, #0x00000001
	moveq	r2, #0x8000
	movteq	r2, #0x0A11
	moveq	r3, #0x0001

	itttt	eq
	movteq	r3, #0xD000
	moveq	r4, #0x76FE
	movteq	r4, #0x59AA
	moveq	r5, #0xB001

	itttt	eq
	movteq	r5, #0x5C37
	moveq	r6, #0x4D1E
	movteq	r6, #0x60B4
	moveq	r7, #0xA556

	ittt	eq
	movteq	r7, #0x9A2C
	moveq	r8, #0x655E
	movteq	r8, #0x12AB

	@ x = ~x

	mvn	 r1,  r1
	mvn	 r2,  r2
	mvn	 r3,  r3
	mvn	 r4,  r4
	mvn	 r5,  r5
	mvn	 r6,  r6
	mvn	 r7,  r7
	mvn	 r8,  r8

	@ x += 1

	adds	 r1, #1
	adcs	 r2, #0
	adcs	 r3, #0
	adcs	 r4, #0
	adcs	 r5, #0
	adcs	 r6, #0
	adcs	 r7, #0
	adc	 r8, #0

	@ x += q

	adds	r1, #0x00000001

	mov	lr, #0x8000;	movt	lr, #0x0A11;	adcs	 r2, lr
	mov	lr, #0x0001;	movt	lr, #0xD000;	adcs	 r3, lr
	mov	lr, #0x76FE;	movt	lr, #0x59AA;	adcs	 r4, lr
	mov	lr, #0xB001;	movt	lr, #0x5C37;	adcs	 r5, lr
	mov	lr, #0x4D1E;	movt	lr, #0x60B4;	adcs	 r6, lr
	mov	lr, #0xA556;	movt	lr, #0x9A2C;	adcs	 r7, lr
	mov	lr, #0x655E;	movt	lr, #0x12AB;	adcs	 r8, lr

	stm	r0, { r1-r8 }
	pop	    { r4-r8, pc }

.size fq_neg, . - fq_neg

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_diff: x = y - z (mod q)
@
@  x = y - z
@  store x
@
@  C = (x < 0); x += q
@  if (C)	/* carry, because x was < 0 */
@    store x
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_diff
.syntax unified
.thumb
.thumb_func
.type fq_diff,	%function

fq_diff:
	push	{ r4-r11, lr }

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

	str	r12, [r0,  #4]
	str	r11, [r0,  #8]
	str	r10, [r0, #12]
	str	 r9, [r0, #16]
	str	 r8, [r0, #20]
	str	 r7, [r0, #24]
	str	 r6, [r0, #28]

	@ x += q, using r1 as temporary, x in { r14, r12-r6 }

	adds	r14, #0x00000001

	mov	r1, #0x8000;	movt	r1, #0x0A11;	adcs	r12, r1
	mov	r1, #0x0001;	movt	r1, #0xD000;	adcs	r11, r1
	mov	r1, #0x76FE;	movt	r1, #0x59AA;	adcs	r10, r1
	mov	r1, #0xB001;	movt	r1, #0x5C37;	adcs	 r9, r1
	mov	r1, #0x4D1E;	movt	r1, #0x60B4;	adcs	 r8, r1
	mov	r1, #0xA556;	movt	r1, #0x9A2C;	adcs	 r7, r1
	mov	r1, #0x655E;	movt	r1, #0x12AB;	adcs	 r6, r1

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

	pop	{ r4-r11, pc }

.size fq_diff, . - fq_diff

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_sum: x = y + z (mod q)
@
@  x = y + z
@  store x
@
@  C = (x >= q); x -= q
@  if (C)
@    store x
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_sum
.syntax unified
.thumb
.thumb_func
.type fq_sum,	%function

fq_sum:
	push	    { r4-r11, lr }

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

	str	r12, [r0,  #4]
	str	r11, [r0,  #8]
	str	r10, [r0, #12]
	str	 r9, [r0, #16]
	str	 r8, [r0, #20]
	str	 r7, [r0, #24]
	str	 r6, [r0, #28]

	@ x -= q, using r1 as temporary, x in { r14, r12-r6 }

	subs	r14, #0x00000001

	mov	r1, #0x8000;	movt	r1, #0x0A11;	sbcs	r12, r1
	mov	r1, #0x0001;	movt	r1, #0xD000;	sbcs	r11, r1
	mov	r1, #0x76FE;	movt	r1, #0x59AA;	sbcs	r10, r1
	mov	r1, #0xB001;	movt	r1, #0x5C37;	sbcs	 r9, r1
	mov	r1, #0x4D1E;	movt	r1, #0x60B4;	sbcs	 r8, r1
	mov	r1, #0xA556;	movt	r1, #0x9A2C;	sbcs	 r7, r1
	mov	r1, #0x655E;	movt	r1, #0x12AB;	sbcs	 r6, r1

	itttt	cs	@ carry set == no borrow => x was >= q => we need to store x
	strcs	r14, [r0,  #0]
	strcs	r12, [r0,  #4]
	strcs	r11, [r0,  #8]
	strcs	r10, [r0, #12]
	itttt	cs
	strcs	 r9, [r0, #16]
	strcs	 r8, [r0, #20]
	strcs	 r7, [r0, #24]
	strcs	 r6, [r0, #28]

	pop	    { r4-r11, pc }

.size fq_sum, . - fq_sum

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_cset: x = c ? y : x
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_cset
.syntax unified
.thumb
.thumb_func
.type fq_cset,	%function

fq_cset:
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

	bx	lr

.size fq_cset, . - fq_cset

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_to_bytes
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_to_bytes
.syntax unified
.thumb
.thumb_func
.type fq_to_bytes,	%function

fq_to_bytes:
				ldr	r2, [r1, #28]
	strb	r2, [r0,  #3];	lsrs	r2, #8
	strb	r2, [r0,  #2];	lsrs	r2, #8
	strb	r2, [r0,  #1];	lsrs	r2, #8
	strb	r2, [r0,  #0];	ldr	r2, [r1, #24]

	strb	r2, [r0,  #7];	lsrs	r2, #8
	strb	r2, [r0,  #6];	lsrs	r2, #8
	strb	r2, [r0,  #5];	lsrs	r2, #8
	strb	r2, [r0,  #4];	ldr	r2, [r1, #20]

	strb	r2, [r0, #11];	lsrs	r2, #8
	strb	r2, [r0, #10];	lsrs	r2, #8
	strb	r2, [r0,  #9];	lsrs	r2, #8
	strb	r2, [r0,  #8];	ldr	r2, [r1, #16]

	strb	r2, [r0, #15];	lsrs	r2, #8
	strb	r2, [r0, #14];	lsrs	r2, #8
	strb	r2, [r0, #13];	lsrs	r2, #8
	strb	r2, [r0, #12];	ldr	r2, [r1, #12]

	strb	r2, [r0, #19];	lsrs	r2, #8
	strb	r2, [r0, #18];	lsrs	r2, #8
	strb	r2, [r0, #17];	lsrs	r2, #8
	strb	r2, [r0, #16];	ldr	r2, [r1,  #8]

	strb	r2, [r0, #23];	lsrs	r2, #8
	strb	r2, [r0, #22];	lsrs	r2, #8
	strb	r2, [r0, #21];	lsrs	r2, #8
	strb	r2, [r0, #20];	ldr	r2, [r1,  #4]

	strb	r2, [r0, #27];	lsrs	r2, #8
	strb	r2, [r0, #26];	lsrs	r2, #8
	strb	r2, [r0, #25];	lsrs	r2, #8
	strb	r2, [r0, #24];	ldr	r2, [r1,  #0]

	strb	r2, [r0, #31];	lsrs	r2, #8
	strb	r2, [r0, #30];	lsrs	r2, #8
	strb	r2, [r0, #29];	lsrs	r2, #8
	strb	r2, [r0, #28]

	bx	lr

.size fq_to_bytes, . - fq_to_bytes

@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@
@ fq_from_bytes
@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

.align 3
.global fq_from_bytes
.syntax unified
.thumb
.thumb_func
.type fq_from_bytes,	%function

fq_from_bytes:
	ldrb	r2, [r1,  #0];	strb	r2, [r0, #31]
	ldrb	r2, [r1,  #1];	strb	r2, [r0, #30]
	ldrb	r2, [r1,  #2];	strb	r2, [r0, #29]
	ldrb	r2, [r1,  #3];	strb	r2, [r0, #28]

	ldrb	r2, [r1,  #4];	strb	r2, [r0, #27]
	ldrb	r2, [r1,  #5];	strb	r2, [r0, #26]
	ldrb	r2, [r1,  #6];	strb	r2, [r0, #25]
	ldrb	r2, [r1,  #7];	strb	r2, [r0, #24]

	ldrb	r2, [r1,  #8];	strb	r2, [r0, #23]
	ldrb	r2, [r1,  #9];	strb	r2, [r0, #22]
	ldrb	r2, [r1, #10];	strb	r2, [r0, #21]
	ldrb	r2, [r1, #11];	strb	r2, [r0, #20]

	ldrb	r2, [r1, #12];	strb	r2, [r0, #19]
	ldrb	r2, [r1, #13];	strb	r2, [r0, #18]
	ldrb	r2, [r1, #14];	strb	r2, [r0, #17]
	ldrb	r2, [r1, #15];	strb	r2, [r0, #16]

	ldrb	r2, [r1, #16];	strb	r2, [r0, #15]
	ldrb	r2, [r1, #17];	strb	r2, [r0, #14]
	ldrb	r2, [r1, #18];	strb	r2, [r0, #13]
	ldrb	r2, [r1, #19];	strb	r2, [r0, #12]

	ldrb	r2, [r1, #20];	strb	r2, [r0, #11]
	ldrb	r2, [r1, #21];	strb	r2, [r0, #10]
	ldrb	r2, [r1, #22];	strb	r2, [r0,  #9]
	ldrb	r2, [r1, #23];	strb	r2, [r0,  #8]

	ldrb	r2, [r1, #24];	strb	r2, [r0,  #7]
	ldrb	r2, [r1, #25];	strb	r2, [r0,  #6]
	ldrb	r2, [r1, #26];	strb	r2, [r0,  #5]
	ldrb	r2, [r1, #27];	strb	r2, [r0,  #4]

	ldrb	r2, [r1, #28];	strb	r2, [r0,  #3]
	ldrb	r2, [r1, #29];	strb	r2, [r0,  #2]
	ldrb	r2, [r1, #30];	strb	r2, [r0,  #1]
	ldrb	r2, [r1, #31];	strb	r2, [r0,  #0]

	bx	lr

.size fq_from_bytes, . - fq_from_bytes

