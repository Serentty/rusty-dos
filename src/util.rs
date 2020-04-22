use rusty_asm::rusty_asm;

pub fn random() -> u16 {
    unsafe {
        rusty_asm! {
            let ax: u16: out("{ax}");
            clobber("bx");
            clobber("cx");
            clobber("dx");
            asm("volatile", "intel") {
                "mov   bx,[11h]
                 mov   ax,bx
                 shl   bx,7
                 xor   ax,bx
                 mov   bx,ax
                 shr   bx,9
                 xor   ax,bx
                 mov   bx,ax
                 shl   bx,8
                 xor   ax,bx
                 mov   [11h],ax"
            }
            return ax;
        }
    }
}

pub fn seed_random() {
    unsafe {
        rusty_asm! {
            clobber("ax");
            clobber("dx");
            clobber("cx");
            asm("volatile", "intel") {
                "mov   ah,0
                 int   1ah
                 mov   [11h],dx
                 xor   dx,dx
                 xor   cx,cx"
            }
        }
    }
}