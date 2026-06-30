cargo install cargo-binutils
cargo readobj -- -h em-rust
/* define memory regiouns */



MEMORY {
    FLASH (rx) : ORIGIN = 0X08000000 , LENGTH = 256K
    RAM (rwx) : ORIGIN =  0X20000000 , LENGTH = 64K
    EEPROM (rwx) : ORIGIN = 0x08080000 , LENGTH = 4K
    CCMRAM (rwx) : ORIGIN = 0X10000000 , LENGHT = 64K
    BATTRAM (rw) : ORIGIN = 0X40024000 , LENGHT = 4K
}