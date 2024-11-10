#![no_std]
#![no_main]

use cortex_m_rt::entry;  // Import de la macro entry
use core::panic::PanicInfo;

#[cfg(feature = "atmega328p")]
mod atmega328p;

#[cfg(feature = "cortex_m")]
mod cortex_m;

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
    main()
}

// Fonction de panic obligatoire dans un environnement no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Fonction principale
#[entry]  // Définition de la fonction main comme point d'entrée
fn main() -> ! {
    #[cfg(feature = "atmega328p")]
    {
        atmega328p::usart_init();
        atmega328p::usart_send(0x41); // Envoi d'un octet 'A'
        atmega328p::usart_send(0x42); // Envoi d'un octet 'B' pour tester si le programme avance
    }

    #[cfg(feature = "cortex_m")]
    {
        cortex_m::usart_init();
        cortex_m::usart_send(0x41); // Envoi d'un octet 'A'
        cortex_m::usart_send(0x42); // Envoi d'un octet 'B' pour tester si le programme avance
    }

    // Boucle infinie avec "blink"
    loop {
        // Envoi d'un octet 'B' périodiquement pour simuler un blink
        #[cfg(feature = "atmega328p")]
        {
            atmega328p::usart_send(0x42); // Envoi de 'B'
        }

        #[cfg(feature = "cortex_m")]
        {
            cortex_m::usart_send(0x42); // Envoi de 'B'
        }

        delay(); // Attendre un moment pour simuler un délai (blink)
    }
}

// Simule un délai
fn delay() {
    // Une boucle qui consomme du temps pour simuler un délai
    for _ in 0..1_000_000 {
        // Simule un léger délai
    }
}
