/* [CORRECTION GPIO] 
    Consider subdividing your project into separate modules. 
(Don't hesitate to remove this comment)*/
#![no_std]
#![no_main]

use core::panic::PanicInfo;

const DDRB: *mut u8 = 0x24 as *mut u8; // Data Direction Register for Port B
const PORTB: *mut u8 = 0x25 as *mut u8; // Port B Data Register
const PINB: *mut u8 = 0x23 as *mut u8;  // Port B Input Pins Address
/* [CORRECTION GPIO] 
    You can only use the I/O registers of port B here (and not the C port for example).
(Don't hesitate to remove this comment)*/

pub enum PinMode {
    Input,
    Output,
}

pub enum PinState {
    Low,
    High,
}

pub struct GPIO {
    pin: u8,
}

impl GPIO {
    // Fonction pour configurer un pin comme entrée ou sortie
    pub fn set_mode(&self, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => {
                    // Configure comme entrée en effaçant le bit correspondant
                    *DDRB &= !(1 << self.pin);
                }
                PinMode::Output => {
                    // Configure comme sortie en mettant le bit correspondant
                    *DDRB |= 1 << self.pin;
                }
            }
        }
    }

    // Fonction pour écrire HIGH ou LOW sur un pin configuré comme sortie
    pub fn write(&self, state: PinState) {
        unsafe {
            match state {
                PinState::High => {
                    *PORTB |= 1 << self.pin; // Met le pin à HIGH
                }
                PinState::Low => {
                    *PORTB &= !(1 << self.pin); // Met le pin à LOW
                }
            }
        }
    }

    // Fonction pour lire l'état d'un pin configuré comme entrée
    pub fn read(&self) -> PinState {
        unsafe {
            if *PINB & (1 << self.pin) != 0 {
                PinState::High
            } else {
                PinState::Low
            }
        }
    }
}

// Fonction de panic obligatoire dans un environnement no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Exemple d'utilisation de l'abstraction GPIO
#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio = GPIO { pin: 5 }; // Utilisation du pin 5 (PB5, connectée à la LED sur Arduino Uno)

    gpio.set_mode(PinMode::Output); // Configure le pin 5 comme sortie
    gpio.write(PinState::High);     // Met le pin 5 à HIGH (allume la LED)

    loop {}
}
