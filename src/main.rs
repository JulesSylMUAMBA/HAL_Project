#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const DDRB: *mut u8 = 0x24 as *mut u8; // Adresse du registre Data Direction Register B
const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre Port B

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Configure le pin 5 du port B comme sortie
        *DDRB |= 1 << 5;
        // Allume la LED (met la sortie à HIGH)
        *PORTB |= 1 << 5;
    }
    loop {}
}
