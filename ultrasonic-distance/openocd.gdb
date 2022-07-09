target extended-remote :3333
set print asm-demangle on
set print pretty on
load
monitor arm semihosting enable
continue