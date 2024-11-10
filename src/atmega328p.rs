const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and Status Register A
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and Status Register C
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low
const UDR0: *mut u8 = 0xC6 as *mut u8;   // USART Data Register

pub fn usart_init() {
    unsafe {
        let baud: u16 = 103; // Baud rate pour 9600 bps avec une horloge de 16 MHz
        *UBRR0H = (baud >> 8) as u8;
        *UBRR0L = baud as u8;

        // Mode 8 bits, sans parité, 1 bit de stop
        *UCSR0C = (1 << 1) | (1 << 2);

        // Activer la transmission et la réception
        *UCSR0B = (1 << 3) | (1 << 4);
    }
}

pub fn usart_send(data: u8) {
    unsafe {
        while *UCSR0A & (1 << 5) == 0 {} // Attendre que le buffer soit prêt
        *UDR0 = data; // Envoyer les données
    }
}
#[allow(dead_code)]
pub fn usart_receive() -> u8 {
    unsafe {
        while *UCSR0A & (1 << 7) == 0 {} // Attendre la réception des données
        *UDR0 // Lire les données reçues
    }
}
