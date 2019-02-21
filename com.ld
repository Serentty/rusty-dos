OUTPUT_FORMAT(binary)
STARTUP(startup.o)
SECTIONS
{
    . = 0x0100;
    .text :
    {
        *(.text);
    }
    .data :
    {
        *(.data);
        *(.bss);
        *(.rodata);
    }
    _heap = ALIGN(4);
}
