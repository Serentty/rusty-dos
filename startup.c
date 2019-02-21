void start(void);

asm (".code16gcc\n"
     "call  start\n"
     "mov   $0x4C,%ah\n"
     "int   $0x21\n");