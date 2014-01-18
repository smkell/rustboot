//===----------------------------------------------------------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is dual licensed under the MIT and the University of Illinois Open
// Source Licenses. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//

// struct { int64_t quot, int64_t rem}
//        __aeabi_ldivmod(int64_t numerator, int64_t denominator) {
//   int64_t rem, quot;
//   quot = __divmoddi4(numerator, denominator, &rem);
//   return {quot, rem};
// }

        .syntax unified
        .align 2
.globl __aeabi_ldivmod
__aeabi_ldivmod:
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      __divmoddi4
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}

        .align 2
.globl __aeabi_uldivmod
__aeabi_uldivmod:
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      __udivmoddi4
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}

.align 3
 .globl __udivsi3
 __udivsi3:
# 51 "udivsi3.S"
    push {r7, lr} ; mov r7, sp
    clz r2, r0
    tst r1, r1
    clz r3, r1
    mov ip, #0
    beq .L_return
    mov lr, #1
    subs r3, r3, r2
    blt .L_return

.L_mainLoop:
# 75 "udivsi3.S"
    subs r2, r0, r1, lsl r3
    itt hs
    orrhs ip, ip,lr, lsl r3
    movhs r0, r2
    it ne
    subsne r3, r3, #1
    bhi .L_mainLoop



    subs r2, r0, r1
    it hs
    orrhs ip, #1

.L_return:

    mov r0, ip
    pop {r7, pc}

.align 3
.globl __umodsi3
 __umodsi3:
# 39 "umodsi3.S"
    clz r2, r0
    tst r1, r1
    clz r3, r1
    bxeq lr
    subs r3, r3, r2
    bxlt lr

.L_mainLoop2:
# 59 "umodsi3.S"
    subs r2, r0, r1, lsl r3
    it hs
    movhs r0, r2
    it ne
    subsne r3, r3, #1
    bhi .L_mainLoop2



    subs r2, r0, r1
    it hs
    movhs r0, r2
    bx lr

.align 3
.globl __aeabi_idiv
__aeabi_idiv:
__divsi3:
# 37 "divsi3.S"
push {r4, r7, lr} ; add r7, sp, #4

    eor r4, r0, r1

    eor r2, r0, r0, asr #31
    eor r3, r1, r1, asr #31
    sub r0, r2, r0, asr #31
    sub r1, r3, r1, asr #31

    bl __udivsi3

    eor r0, r0, r4, asr #31
    sub r0, r0, r4, asr #31
    pop {r4, r7, pc}

.align 3
 ; .globl __modsi3 ; __modsi3:
# 36 "modsi3.S"
    push {r4, r7, lr} ; add r7, sp, #4

    mov r4, r0

    eor r2, r0, r0, asr #31
    eor r3, r1, r1, asr #31
    sub r0, r2, r0, asr #31
    sub r1, r3, r1, asr #31

    bl __umodsi3

    eor r0, r0, r4, asr #31
    sub r0, r0, r4, asr #31
    pop {r4, r7, pc}
