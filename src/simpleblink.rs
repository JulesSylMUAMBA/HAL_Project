use crate::token_stream; // Importation de token_stream


pub fn blink() -> ! {
    unsafe {
        // Initialisation des registres pour la LED
        core::ptr::write_volatile(0x25 as *mut u8, 0b00110000);  // DDRB: 0b00110000 (sortie pour PB4 et PB5)
        loop {
            // Allumer la LED
            for _ in 0..2_000_000 {
                core::ptr::write_volatile(0x25 as *mut u8, 0b00100000);  // PORTB: 0b00100000 (allumer PB5)
            }
            token_stream::send_message("on");  // Envoi du message "on" via UART

            // Éteindre la LED
            for _ in 0..100_000 {
                core::ptr::write_volatile(0x25 as *mut u8, 0b00000000);  // PORTB: 0b00000000 (éteindre PB5)
            }
            token_stream::send_message("off");  // Envoi du message "off" via UART
        }
    }
}