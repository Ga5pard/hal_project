#![no_std]
#![no_main]

use cortex_m_rt::entry; // Utilise l'attribut #[entry] pour spécifier le point d'entrée

// Déclarations des adresses des registres pour le port B de l'Atmega328p
const DDRB: *mut u8 = 0x24 as *mut u8;  // Registre de direction (entrée/sortie) pour PORTB
const PORTB: *mut u8 = 0x25 as *mut u8; // Registre de données pour définir l'état des broches (HIGH/LOW)
const PINB: *const u8 = 0x23 as *const u8; // Registre pour lire l'état des broches configurées en entrée

/// Structure représentant une broche GPIO.
pub struct GPIO {
    pub pin: u8, // Le numéro de la broche (0 à 7) correspondant à PORTB
}

impl GPIO {
    /// Initialise une nouvelle broche GPIO en tant qu'entrée ou sortie selon le paramètre `output`.
    /// - 'pin': Numéro de la broche
    /// - 'output': 'true' pour configurer la broche en sortie, 'false' pour entrée
    pub unsafe fn new(pin: u8, output: bool) -> Self {
        config_pin(pin, output); // Configure la broche en entrée/sortie
        GPIO { pin } // Renvoie une instance de la structure GPIO
    }

    /// Configure la broche comme sortie ('output = true') ou entrée ('output = false').
    /// Permet de changer la direction de la broche après l'initialisation.
    pub unsafe fn set_mode(&self, output: bool) {
        config_pin(self.pin, output);
    }

    /// Met la broche à l'état 'HIGH' pour envoyer une tension sur la broche.
    pub unsafe fn set_high(&self) {
        write_pin(self.pin, true);
    }

    /// Met la broche à l'état 'LOW' pour couper la tension sur la broche.
    pub unsafe fn set_low(&self) {
        write_pin(self.pin, false);
    }

    /// Vérifie si la broche est à 'HIGH'.
    /// Retourne 'true' si la broche est à 'HIGH', 'false' sinon.
    pub unsafe fn is_high(&self) -> bool {
        read_pin(self.pin)
    }
}

/// Fonction interne pour configurer une broche de PORTB comme entrée ou sortie.
/// - 'pin': Numéro de la broche
/// - 'output': 'true' pour sortie, 'false' pour entrée
unsafe fn config_pin(pin: u8, output: bool) {
    if output {
        // Configure la broche en sortie en définissant le bit correspondant à 1 dans DDRB
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | (1 << pin));
    } else {
        // Configure la broche en entrée en mettant le bit correspondant à 0 dans DDRB
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) & !(1 << pin));
    }
}

/// Fonction interne pour mettre une broche de PORTB à l'état 'HIGH' ou 'LOW'
/// - 'pin': Numéro de la broche
/// - 'high': 'true' pour 'HIGH', 'false' pour 'LOW'
unsafe fn write_pin(pin: u8, high: bool) {
    if high {
        // Met la broche à 'HIGH' en activant le bit correspondant dans PORTB
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) | (1 << pin));
    } else {
        // Met la broche à 'LOW' en désactivant le bit correspondant dans PORTB
        core::ptr::write_volatile(PORTB, core::ptr::read_volatile(PORTB) & !(1 << pin));
    }
}

/// Fonction interne pour lire l'état d'une broche de PORTB.
/// - 'pin': Numéro de la broche
/// Retourne 'true' si la broche est à 'HIGH', 'false' si 'LOW'
unsafe fn read_pin(pin: u8) -> bool {
    // Vérifie l'état du bit correspondant dans PINB pour déterminer si la broche est à HIGH ou LOW
    (core::ptr::read_volatile(PINB) & (1 << pin)) != 0
}

#[entry]
fn main() -> ! {
    unsafe {
        // Crée une instance de GPIO pour la broche 5 et la configure en sortie
        let led = GPIO::new(5, true);

        loop {
            // Allume la LED connectée à la broche 5 en mettant la broche à HIGH
            led.set_high();
            for _ in 0..1_000_000 {} // Délai pour maintenir la LED allumée

            // Éteint la LED connectée à la broche 5 en mettant la broche à LOW
            led.set_low();
            for _ in 0..1_000_000 {} // Délai pour maintenir la LED éteinte
        }
    }
}

/// Gestionnaire de panique requis pour un environnement 'no_std'.
/// Entre dans une boucle infinie en cas de panique pour éviter de redémarrer ou planter le programme.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
