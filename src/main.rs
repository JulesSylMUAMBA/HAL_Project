/*
[CORRECTION USART] (Don't hesitate to remove this part)
You didn't implement the USART feature.

[CORRECTION SPI] (don't hesitate to remove this part)
You should implement the peripheral/slave mode as well (not only the controler/master mode).
*/

#![no_std]
#![no_main]

use cortex_m_rt::entry; // Import de la macro entry
use core::panic::PanicInfo;

#[cfg(feature = "atmega328p")]
mod atmega328p;

#[cfg(feature = "cortex_m")]
mod cortex_m;

#[cfg(feature = "atmega328p")]
mod atmega328p_i2c;

#[cfg(feature = "cortex_m")]
mod cortex_m_i2c;

// Pointeur de pile initial, placé à l'adresse de fin de la RAM
const INITIAL_SP: u32 = 0x2001_0000; // 64 Ko de SRAM pour lm3s6965evb

// Table de vecteurs pour Cortex-M
#[link_section = ".vectors"]
#[no_mangle]
pub static VECTORS: &[Option<unsafe extern "C" fn()>; 2] = &[
    Some(start_stack),   // Adresse initiale de la pile
    Some(reset_wrapper), // Pointeur de réinitialisation
];

// Fonction de démarrage de la pile
#[no_mangle]
pub extern "C" fn start_stack() {
    // Cette fonction reste vide, utilisée uniquement comme référence pour la table de vecteurs.
}

// Fonction intermédiaire pour appeler le gestionnaire de réinitialisation
unsafe extern "C" fn reset_wrapper() {
    reset_handler()
}

// Fonction de réinitialisation
#[no_mangle]
pub extern "C" fn reset_handler() -> ! {
    run_application() // Appelle la logique principale de ton application
}

// Fonction de panic obligatoire dans un environnement no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Fonction principale annotée avec #[entry] pour définir le point d'entrée
#[entry]
fn main() -> ! {
    run_application()
}

// Logique principale de l'application
fn run_application() -> ! {
    #[cfg(feature = "atmega328p")]
    {
        // Initialisation SPI pour l'Atmega328p
        atmega328p::spi_init();

        // Initialisation I2C pour l'Atmega328p
        atmega328p_i2c::i2c_init();

        // Envoi d'un octet via SPI
        atmega328p::spi_send(0xAB); // Exemple d'envoi
        atmega328p::spi_send(0xCD); // Autre octet

        // Envoi d'un octet via I2C
        atmega328p_i2c::i2c_send(0x01);

        // Test réception I2C
        let received = atmega328p_i2c::i2c_receive();
        atmega328p::spi_send(received); // Renvoi via SPI pour vérification

        loop {
            atmega328p::spi_send(0x55); // Envoi continu du caractère 'U'
            delay();
        }
    }

    #[cfg(feature = "cortex_m")]
    {
        // Initialisation SPI pour le Cortex-M
        cortex_m::spi_init();

        // Initialisation I2C pour le Cortex-M
        cortex_m_i2c::i2c_init();

        // Envoi d'un octet via SPI
        cortex_m::spi_send(0xAB); // Exemple d'envoi
        cortex_m::spi_send(0xCD); // Autre octet

        // Envoi d'un octet via I2C
        cortex_m_i2c::i2c_send(0x01);

        // Test réception I2C
        let received = cortex_m_i2c::i2c_receive();
        cortex_m::spi_send(received); // Renvoi via SPI pour vérification
    }

    // Boucle infinie avec "blink"
    loop {
        #[cfg(feature = "atmega328p")]
        {
            atmega328p::spi_send(0x42); // Envoi de 'B' via SPI
        }

        #[cfg(feature = "cortex_m")]
        {
            cortex_m::spi_send(0x42); // Envoi de 'B' via SPI
        }

        delay(); // Attendre un moment pour simuler un délai (blink)
    }
}

// Simule un délai
fn delay() {
    for _ in 0..1_000_000 {
        // Simule un léger délai
    }
}
