# Supported Instructions

Moras supports all instruction in RV32i. Fence instructions do nothing as Moras doesn't have a cache.

If there is an instruction from RV32i missing, that's a bug. Additionally, all pseudo-instructions listed in the spec
should also be supported.


| **Example Usage**                   | **Description**                                                                                                                                              |
|:------------------------------------|:-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| add&nbsp;t1,t2,t3 	                 | Addition: set t1 to (t2 plus t3)                                                                                                                             |
| addi&nbsp;t1,t2,-100	               | Addition immediate: set t1 to (t2 plus signed 12-bit immediate)                                                                                              |
| and&nbsp;t1,t2,t3	                  | Bitwise AND : Set t1 to bitwise AND of t2 and t3                                                                                                             |
| andi&nbsp;t1,t2,-100	               | Bitwise AND immediate : Set t1 to bitwise AND of t2 and sign-extended 12-bit immediate                                                                       |
| auipc&nbsp;t1,10000	                | Add upper immediate to pc: set t1 to (pc plus an upper 20-bit immediate)                                                                                     |
| beq&nbsp;t1,t2,label	               | Branch if equal : Branch to statement at label's address if t1 and t2 are equal                                                                              |
| bge&nbsp;t1,t2,label	               | Branch if greater than or equal: Branch to statement at label's address if t1 is greater than or equal to t2                                                 |
| bgeu&nbsp;t1,t2,label	              | Branch if greater than or equal to (unsigned): Branch to statement at label's address if t1 is greater than or equal to t2 (with an unsigned interpretation) |
| blt&nbsp;t1,t2,label	               | Branch if less than: Branch to statement at label's address if t1 is less than t2                                                                            |
| bltu&nbsp;t1,t2,label	              | Branch if less than (unsigned): Branch to statement at label's address if t1 is less than t2 (with an unsigned interpretation)                               |
| bne&nbsp;t1,t2,label	               | Branch if not equal : Branch to statement at label's address if t1 and t2 are not equal                                                                      |
| csrrc&nbsp;t0,&nbsp;fcsr,&nbsp;t1	  | Atomic Read/Clear CSR: read from the CSR into t0 and clear bits of the CSR according to t1                                                                   |
| csrrci&nbsp;t0,&nbsp;fcsr,&nbsp;10	 | Atomic Read/Clear CSR Immediate: read from the CSR into t0 and clear bits of the CSR according to a constant                                                 |
| csrrs&nbsp;t0,&nbsp;fcsr,&nbsp;t1	  | Atomic Read/Set CSR: read from the CSR into t0 and logical or t1 into the CSR                                                                                |
| csrrsi&nbsp;t0,&nbsp;fcsr,&nbsp;10	 | Atomic Read/Set CSR Immediate: read from the CSR into t0 and logical or a constant into the CSR                                                              |
| csrrw&nbsp;t0,&nbsp;fcsr,&nbsp;t1	  | Atomic Read/Write CSR: read from the CSR into t0 and write t1 into the CSR                                                                                   |
| csrrwi&nbsp;t0,&nbsp;fcsr,&nbsp;10	 | Atomic Read/Write CSR Immediate: read from the CSR into t0 and write a constant into the CSR                                                                 |
| ebreak	                             | Pause execution                                                                                                                                              |
| ecall	                              | Issue a system call : Execute the system call specified by value in a7                                                                                       |
| fence&nbsp;1,&nbsp;1	               | Ensure that IO and memory accesses before the fence happen before the following IO and memory accesses as viewed by a different thread                       |
| fence.i	                            | Ensure that stores to instruction memory are visible to instruction fetches                                                                                  |
| jal&nbsp;t1,&nbsp;target	           | Jump and link : Set t1 to Program Counter (return address) then jump to statement at target address                                                          |
| jalr&nbsp;t1,&nbsp;t2,&nbsp;-100	   | Jump and link register: Set t1 to Program Counter (return address) then jump to statement at t2 + immediate                                                  |
| lb&nbsp;t1,&nbsp;-100(t2)	          | Set t1 to sign-extended 8-bit value from effective memory byte address                                                                                       |
| lbu&nbsp;t1,&nbsp;-100(t2)	         | Set t1 to zero-extended 8-bit value from effective memory byte address                                                                                       |
| lh&nbsp;t1,&nbsp;-100(t2)	          | Set t1 to sign-extended 16-bit value from effective memory halfword address                                                                                  |
| lhu&nbsp;t1,&nbsp;-100(t2)	         | Set t1 to zero-extended 16-bit value from effective memory halfword address                                                                                  |
| lui&nbsp;t1,10000	                  | Load upper immediate: set t1 to 20-bit followed by 12 0s                                                                                                     |
| lw&nbsp;t1,&nbsp;-100(t2)	          | Set t1 to contents of effective memory word address                                                                                                          |
| or&nbsp;t1,t2,t3	                   | Bitwise OR : Set t1 to bitwise OR of t2 and t3                                                                                                               |
| ori&nbsp;t1,t2,-100	                | Bitwise OR immediate : Set t1 to bitwise OR of t2 and sign-extended 12-bit immediate                                                                         |
| sb&nbsp;t1,&nbsp;-100(t2)	          | Store byte : Store the low-order 8 bits of t1 into the effective memory byte address                                                                         |
| sh&nbsp;t1,&nbsp;-100(t2)	          | Store halfword : Store the low-order 16 bits of t1 into the effective memory halfword address                                                                |
| sll&nbsp;t1,t2,t3	                  | Shift left logical: Set t1 to result of shifting t2 left by number of bits specified by value in low-order 5 bits of t3                                      |
| slli&nbsp;t1,t2,10	                 | Shift left logical : Set t1 to result of shifting t2 left by number of bits specified by immediate                                                           |
| slt&nbsp;t1,t2,t3	                  | Set less than : If t2 is less than t3, then set t1 to 1 else set t1 to 0                                                                                     |
| slti&nbsp;t1,t2,-100	               | Set less than immediate : If t2 is less than sign-extended 12-bit immediate, then set t1 to 1 else set t1 to 0                                               |
| sltiu&nbsp;t1,t2,-100	              | Set less than immediate unsigned : If t2 is less than sign-extended 16-bit immediate using unsigned comparison, then set t1 to 1 else set t1 to 0            |
| sltu&nbsp;t1,t2,t3	                 | Set less than : If t2 is less than t3 using unsigned comparision, then set t1 to 1 else set t1 to 0                                                          |
| sra&nbsp;t1,t2,t3	                  | Shift right arithmetic: Set t1 to result of sign-extended shifting t2 right by number of bits specified by value in low-order 5 bits of t3                   |
| srai&nbsp;t1,t2,10	                 | Shift right arithmetic : Set t1 to result of sign-extended shifting t2 right by number of bits specified by immediate                                        |
| srl&nbsp;t1,t2,t3	                  | Shift right logical: Set t1 to result of shifting t2 right by number of bits specified by value in low-order 5 bits of t3                                    |
| srli&nbsp;t1,t2,10	                 | Shift right logical : Set t1 to result of shifting t2 right by number of bits specified by immediate                                                         |
| sub&nbsp;t1,t2,t3	                  | Subtraction: set t1 to (t2 minus t3)                                                                                                                         |
| sw&nbsp;t1,&nbsp;-100(t2)	          | Store word : Store contents of t1 into effective memory word address                                                                                         |
| xor&nbsp;t1,t2,t3	                  | Bitwise XOR : Set t1 to bitwise XOR of t2 and t3                                                                                                             |
| xori&nbsp;t1,t2,-100	               | Bitwise XOR immediate : Set t1 to bitwise XOR of t2 and sign-extended 12-bit immediate                                                                       |

**Supported psuedo-instructions**:

| **Example Usage**           | **Description**                                                                                                       |
|:----------------------------|:----------------------------------------------------------------------------------------------------------------------|
| addi&nbsp;t1,t2,%lo(label)	 | Load Lower Address : Set t1 to t2 + lower 12-bit label's address                                                      |
| b&nbsp;label	               | Branch : Branch to statement at label unconditionally                                                                 |
| beqz&nbsp;t1,label	         | Branch if EQual Zero : Branch to statement at label if t1 == 0                                                        |
| bgez&nbsp;t1,label	         | Branch if Greater than or Equal to Zero : Branch to statement at label if t1 >= 0                                     |
| bgt&nbsp;t1,t2,label	       | Branch if Greater Than : Branch to statement at label if t1 > t2                                                      |
| bgtu&nbsp;t1,t2,label	      | Branch if Greater Than Unsigned: Branch to statement at label if t1 > t2 (unsigned compare)                           |
| bgtz&nbsp;t1,label	         | Branch if Greater Than: Branch to statement at label if t1 > 0                                                        |
| ble&nbsp;t1,t2,label	       | Branch if Less or Equal : Branch to statement at label if t1 <= t2                                                    |
| bleu&nbsp;t1,t2,label	      | Branch if Less or Equal Unsigned : Branch to statement at label if t1 <= t2 (unsigned compare)                        |
| blez&nbsp;t1,label	         | Branch if Less than or Equal to Zero : Branch to statement at label if t1 <= 0                                        |
| bltz&nbsp;t1,label	         | Branch if Less Than Zero : Branch to statement at label if t1 < 0                                                     |
| bnez&nbsp;t1,label	         | Branch if Not Equal Zero : Branch to statement at label if t1 != 0                                                    |
| call&nbsp;label	            | CALL: call a far-away subroutine                                                                                      |
| csrc&nbsp;t1,&nbsp;fcsr	    | Clear bits in control and status register                                                                             |
| csrci&nbsp;fcsr,&nbsp;100	  | Clear bits in control and status register                                                                             |
| csrr&nbsp;t1,&nbsp;fcsr	    | Read control and status register                                                                                      |
| csrs&nbsp;t1,&nbsp;fcsr	    | Set bits in control and status register                                                                               |
| csrsi&nbsp;fcsr,&nbsp100	   | Set bits in control and status register                                                                               |
| csrw&nbsp;t1,&nbsp;fcsr	    | Write control and status register                                                                                     |
| csrwi&nbsp;fcsr,&nbsp;100	  | Write control and status register                                                                                     |
| j&nbsp;label	               | Jump : Jump to statement at label                                                                                     |
| jal&nbsp;label	             | Jump And Link: Jump to statement at label and set the return address to ra                                            |
| jalr&nbsp;t0                | Jump And Link Register: Jump to address in t0 and set the return address to ra                                        |
| jalr&nbsp;t0,&nbsp;-100	    | Jump And Link Register: Jump to address in t0 and set the return address to ra                                        |
| jr&nbsp;t0	                 | Jump Register: Jump to address in t0                                                                                  |
| jr&nbsp;t0,&nbsp;-100	      | Jump Register: Jump to address in t0                                                                                  |
| la&nbsp;t1,label	           | Load Address : Set t1 to label's address                                                                              |
| lb&nbsp;t1,(t2)	            | Load Byte : Set t1 to sign-extended 8-bit value from effective memory byte address                                    |
| lb&nbsp;t1,-100	            | Load Byte : Set $1 to sign-extended 8-bit value from effective memory byte address                                    |
| lb&nbsp;t1,10000000	        | Load Byte : Set $t1 to sign-extended 8-bit value from effective memory byte address                                   |
| lb&nbsp;t1,label	           | Load Byte : Set $t1 to sign-extended 8-bit value from effective memory byte address                                   |
| lbu&nbsp;t1,(t2)	           | Load Byte Unsigned : Set $t1 to zero-extended 8-bit value from effective memory byte address                          |
| lbu&nbsp;t1,-100	           | Load Byte Unsigned : Set $t1 to zero-extended 8-bit value from effective memory byte address                          |
| lbu&nbsp;t1,10000000	       | Load Byte Unsigned : Set t1 to zero-extended 8-bit value from effective memory byte address                           |
| lbu&nbsp;t1,label	          | Load Byte Unsigned : Set t1 to zero-extended 8-bit value from effective memory byte address                           |
| lh&nbsp;t1,(t2)	            | Load Halfword : Set t1 to sign-extended 16-bit value from effective memory halfword address                           |
| lh&nbsp;t1,-100	            | Load Halfword : Set t1 to sign-extended 16-bit value from effective memory halfword address                           |
| lh&nbsp;t1,10000000         | 	Load Halfword : Set t1 to sign-extended 16-bit value from effective memory halfword address                          |
| lh&nbsp;t1,label	           | Load Halfword : Set t1 to sign-extended 16-bit value from effective memory halfword address                           |
| lhu&nbsp;t1,(t2)	           | Load Halfword Unsigned : Set t1 to zero-extended 16-bit value from effective memory halfword address                  |
| lhu&nbsp;t1,-100	           | Load Halfword Unsigned : Set t1 to zero-extended 16-bit value from effective memory halfword address                  |
| lhu&nbsp;t1,10000000	       | Load Halfword Unsigned : Set t1 to zero-extended 16-bit value from effective memory halfword address                  |
| lhu&nbsp;t1,label	          | Load Halfword Unsigned : Set t1 to zero-extended 16-bit value from effective memory halfword address                  |
| li&nbsp;t1,-100	            | Load Immediate : Set t1 to 12-bit immediate (sign-extended)                                                           |
| li&nbsp;t1,10000000	        | Load Immediate : Set t1 to 32-bit immediate                                                                           |
| lui&nbsp;t1,%hi(label)	     | Load Upper Address : Set t1 to upper 20-bit label's address                                                           |
| lw&nbsp;t1,%lo(label)(t2)	  | Load from Address                                                                                                     |
| lw&nbsp;t1,(t2)	            | Load Word : Set t1 to contents of effective memory word address                                                       |
| lw&nbsp;t1,-100	            | Load Word : Set t1 to contents of effective memory word address                                                       |
| lw&nbsp;t1,10000000	        | Load Word : Set t1 to contents of effective memory word address                                                       |
| lw&nbsp;t1,label	           | Load Word : Set t1 to contents of memory word at label's address                                                      |
| mv&nbsp;t1,t2	              | MoVe : Set t1 to contents of t2                                                                                       |
| neg&nbsp;t1,t2	             | NEGate : Set t1 to negation of t2                                                                                     |
| nop	                        | NO OPeration                                                                                                          |
| not&nbsp;t1,t2	             | Bitwise NOT (bit inversion)                                                                                           |
| sb&nbsp;t1,(t2)	            | Store Byte : Store the low-order 8 bits of t1 into the effective memory byte address                                  |
| sb&nbsp;t1,-100	            | Store Byte : Store the low-order 8 bits of $t1 into the effective memory byte address                                 |
| sb&nbsp;t1,10000000,t2	     | Store Byte : Store the low-order 8 bits of $t1 into the effective memory byte address                                 |
| sb&nbsp;t1,label,t2	        | Store Byte : Store the low-order 8 bits of $t1 into the effective memory byte address                                 |
| seqz&nbsp;t1,t2	            | Set EQual to Zero : if t2 == 0 then set t1 to 1 else 0                                                                |
| sgt&nbsp;t1,t2,t3	          | Set Greater Than : if t2 greater than t3 then set t1 to 1 else 0                                                      |
| sgtu&nbsp;t1,t2,t3	         | Set Greater Than Unsigned : if t2 greater than t3 (unsigned compare) then set t1 to 1 else 0                          |
| sgtz&nbsp;t1,t2	            | Set Greater Than Zero : if t2 > 0 then set t1 to 1 else 0                                                             |
| sh&nbsp;t1,(t2)	            | Store Halfword : Store the low-order 16 bits of $1 into the effective memory halfword address                         |
| sh&nbsp;t1,-100	            | Store Halfword : Store the low-order 16 bits of $t1 into the effective memory halfword address                        |
| sh&nbsp;t1,10000000,t2	     | Store Halfword : Store the low-order 16 bits of t1 into the effective memory halfword address using t2 as a temporary |
| sh&nbsp;t1,label,t2	        | Store Halfword : Store the low-order 16 bits of t1 into the effective memory halfword address using t2 as a temporary |
| sltz&nbsp;t1,t2	            | Set Less Than Zero : if t2 < 0 then set t1 to 1 else 0                                                                |
| snez&nbsp;t1,t2	            | Set Not Equal to Zero : if t2 != 0 then set t1 to 1 else 0                                                            |
| sw&nbsp;t1,(t2)	            | Store Word : Store t1 contents into effective memory word address                                                     |
| sw&nbsp;t1,-100	            | Store Word : Store $t1 contents into effective memory word address                                                    |
| sw&nbsp;t1,10000000,t2	     | Store Word : Store $t1 contents into effective memory word address using t2 as a temporary                            |
| sw&nbsp;t1,label,t2	        | Store Word : Store $t1 contents into memory word at label's address using t2 as a temporary                           |

Cited from [RARS wiki](https://github.com/TheThirdOne/rars/wiki/Supported-Instructions)