// Registres pour le SPI1 sur le STM32 (Cortex-M)
const SPI1_CR1: *mut u32 = 0x4001_3000 as *mut u32; // SPI Control Register 1
const SPI1_CR2: *mut u32 = 0x4001_3004 as *mut u32; // SPI Control Register 2
const SPI1_SR: *mut u32 = 0x4001_3008 as *mut u32;  // SPI Status Register
const SPI1_DR: *mut u32 = 0x4001_300C as *mut u32;  // SPI Data Register
const SPI1_BDR: *mut u32 = 0x4001_3010 as *mut u32; // Baud Rate Control

// Définition des bits de SPI_CR1
const SPI_ENABLE: u32 = 0x40; // Enable SPI
const SPI_MASTER: u32 = 0x4;  // Master mode
const SPI_SPE: u32 = 0x40;   // SPI Enable
const SPI_MSTR: u32 = 0x4;   // Master selection
const SPI_BR: u32 = 0x3;     // Baud rate control (Fosc/16)

// Fonction d'initialisation SPI pour le Cortex-M
pub fn spi_init() {
    unsafe {
        // Désactivation de SPI pour pouvoir le configurer
        *SPI1_CR1 &= !SPI_SPE;

        // Configuration de SPI : Mode maître, Baud rate, etc.
        *SPI1_CR1 = SPI_MSTR | SPI_SPE | SPI_BR; // Activer SPI en mode maître avec un baud rate Fosc/16

        // Configurer le registre SPI_CR2 pour des paramètres supplémentaires (si nécessaire)

        // Activation de SPI
        *SPI1_CR1 |= SPI_SPE; // Activer le SPI
    }
}

// Fonction pour envoyer un octet via SPI
pub fn spi_send(data: u8) {
    unsafe {
        // Attente que le registre de transmission soit vide
        while *SPI1_SR & 0x02 == 0 {} // Vérifier le bit TXE (Transmission FIFO Empty)

        // Charger les données à envoyer dans le registre SPI_DR
        *SPI1_DR = data as u32;

        // Attente que la transmission soit terminée
        while *SPI1_SR & 0x80 == 0 {} // Attendre que le bit SPIF soit set
    }
}

// Fonction pour recevoir un octet via SPI
pub fn spi_receive() -> u8 {
    unsafe {
        // Attente que le registre de réception soit plein
        while *SPI1_SR & 0x01 == 0 {} // Vérifier le bit RXNE (Reception FIFO Not Empty)

        // Lire les données reçues dans le registre SPI_DR
        *SPI1_DR as u8
    }
}
