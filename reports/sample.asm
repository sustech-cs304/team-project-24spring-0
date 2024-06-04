.data
_s0:
        .string "before:"
_s1:
        .string "after:"
arr:
        .word   416
        .word   8956
        .word   8764
        .word   1654
        .word   8654
        .word   6853478
        .word   8904
        .word   -408
        .word   -5
        .word   656
n:
        .word   10

.text
main:
        addi    sp,sp,-32
        sw      ra,28(sp)
        sw      s0,24(sp)
        addi    s0,sp,32
        la      a0,_s0
        li      a7,4
        ecall
        li      a0,0x0A
        li      a7,11
        ecall
        sw      zero,-28(s0)
        j       L9
L10:
        la      a4,arr
        lw      a5,-28(s0)
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a0,0(a5)
        li      a7,1
        ecall
        li      a0,0x20
        li      a7,11
        ecall
        lw      a5,-28(s0)
        addi    a5,a5,1
        sw      a5,-28(s0)
L9:
        lw      a5,n
        lw      a4,-28(s0)
        blt     a4,a5,L10
        li      a0,0x0A
        li      a7,11
        ecall
        sw      zero,-20(s0)
        j       L2
L6:
        sw      zero,-24(s0)
        j       L3
L5:
        la      a4,arr
        lw      a5,-24(s0)
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a4,0(a5)
        lw      a5,-24(s0)
        addi    a5,a5,1
        la      a3,arr
        slli    a5,a5,2
        add     a5,a3,a5
        lw      a5,0(a5)
        ble     a4,a5,L4
        la      a4,arr
        lw      a5,-24(s0)
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a5,0(a5)
        sw      a5,-28(s0)
        lw      a5,-24(s0)
        addi    a5,a5,1
        la      a4,arr
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a4,0(a5)
        la      a3,arr
        lw      a5,-24(s0)
        slli    a5,a5,2
        add     a5,a3,a5
        sw      a4,0(a5)
        lw      a5,-24(s0)
        addi    a5,a5,1
        la      a4,arr
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a4,-28(s0)
        sw      a4,0(a5)
L4:
        lw      a5,-24(s0)
        addi    a5,a5,1
        sw      a5,-24(s0)
L3:
        lw      a4,n
        lw      a5,-20(s0)
        sub     a5,a4,a5
        addi    a5,a5,-1
        lw      a4,-24(s0)
        blt     a4,a5,L5
        lw      a5,-20(s0)
        addi    a5,a5,1
        sw      a5,-20(s0)
L2:
        lw      a5,n
        addi    a5,a5,-1
        lw      a4,-20(s0)
        blt     a4,a5,L6
        la      a0,_s1
        li      a7,4
        ecall
        li      a0,0x0A
        li      a7,11
        ecall
        sw      zero,-28(s0)
        j       L7
L8:
        la      a4,arr
        lw      a5,-28(s0)
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a0,0(a5)
        li      a7,1
        ecall
        li      a0,0x20
        li      a7,11
        ecall
        lw      a5,-28(s0)
        addi    a5,a5,1
        sw      a5,-28(s0)
L7:
        lw      a5,n
        lw      a4,-28(s0)
        blt     a4,a5,L8
        li      a0,0x0A
        li      a7,11
        ecall
        li      a5,0
        mv      a0,a5
        lw      ra,28(sp)
        lw      s0,24(sp)
        addi    sp,sp,32