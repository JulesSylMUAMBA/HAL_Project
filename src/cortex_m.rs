// Définitions des registres UART0 pour le modèle lm3s6965evb
const UART0_DR: *mut u32 = 0x4000_C000 as *mut u32;   // UART Data Register
const UART0_FR: *mut u32 = 0x4000_C018 as *mut u32;   // UART Flag Register
const UART0_IBRD: *mut u32 = 0x4000_C024 as *mut u32; // Integer Baud Rate Divisor
const UART0_FBRD: *mut u32 = 0x4000_C028 as *mut u32; // Fractional Baud Rate Divisor
const UART0_LCRH: *mut u32 = 0x4000_C02C as *mut u32; // Line Control
const UART0_CTL: *mut u32 = 0x4000_C030 as *mut u32;  // UART Control

pub fn usart_init() {
    unsafe {
        // Désactivez l'UART pendant la configuration
        *UART0_CTL = 0;

        // Configurer le baud rate (9600 bps avec un horloge de 16 MHz)
        *UART0_IBRD = 104; // Valeur entière pour le diviseur de baud rate
        *UART0_FBRD = 11;  // Valeur fractionnelle pour le diviseur de baud rate

        // Configurer la ligne (8 bits, pas de parité, 1 bit d'arrêt)
        *UART0_LCRH = (0x3 << 5); // 8 bits, pas de parité, 1 bit d'arrêt

        // Activer UART0, RX, et TX
        *UART0_CTL = (1 << 0) | (1 << 8) | (1 << 9);
    }
}

pub fn usart_send(data: u8) {
    unsafe {
        // Attendre que le FIFO de transmission soit prêt
        while (*UART0_FR & (1 << 5)) != 0 {}
        *UART0_DR = data as u32;
    }
}
