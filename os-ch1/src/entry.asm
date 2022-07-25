   .section .text.entry    # 下方全部放在 .text.entry 中，目的是确保位于最低的地址以最先执行
   .globl _start   # 全局符号，可以被其他目标文件使用
 _start:
   # li x1, 100  # 真实的第一条指令
   la sp, boot_stack_top
   call rust_main

   .section .bss.stack
   .globl stack
boot_stack: # 栈底
   .space 4096*16 # 64KB 的栈空间
   .global boot_stack_top
boot_stack_top:   # 栈顶

