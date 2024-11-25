const SPCR: *mut u8 = 0x2C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x2E as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x2F as *mut u8; // SPI Data Register

// Définir les bits de SPCR
const SPI_ENABLE: u8 = 0x40; // SPI Enable
const SPI_MASTER: u8 = 0x10; // Master Mode
const SPI_CLK_DIV16: u8 = 0x03; // Clock Divider Fosc/16

pub fn spi_init() {
    unsafe {
        
        *SPCR = SPI_ENABLE | SPI_MASTER | SPI_CLK_DIV16; 
    }
}

pub fn spi_send(data: u8) {
    unsafe {
        // Charger les données à envoyer dans le registre SPDR
        *SPDR = data;

        // Attendre que la transmission soit terminée (vérifier le bit SPIF dans le registre SPSR)
        while *SPSR & 0x80 == 0 {} // Attendre la fin de la transmission
    }
}

pub fn spi_receive() -> u8 {
    unsafe {
        // Attendre que les données aient été reçues
        while *SPSR & 0x80 == 0 {} // Attendre que SPIF soit à 1, signalant que les données sont prêtes

        // Retourner les données lues dans le registre SPDR
        *SPDR
    }
}
