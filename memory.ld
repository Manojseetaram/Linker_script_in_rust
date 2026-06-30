/* Define memory regions */

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 512K
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;

SECTIONS 
{
    .text  : 
    {
    . = ALIGN(4);
    *(.text)
    *(.text.*)
    . = ALIGN(4);
    } > FLASH 

    .rodata :
    { 
    . = ALIGN(4);
   *(.rodata)
   *(.rodata*)
    . = ALIGN(4);
    } > FLASH
    .data :
    {
        _sidata = LOADADDR(.data);
            . = ALIGN(4);
        _sdata = .;
      *(.data)
      *(.data*)
    . = ALIGN(4);
      _edata = .;
    } > RAM AT> FLASH
    .bss :
    {
            . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
            . = ALIGN(4);
        _ebss = .;
    } > RAM


    .ram_usage_check : 
    {
        . = . + _min_stack_size ;
        . = . + _min_heap_size ;
    } > RAM
}
