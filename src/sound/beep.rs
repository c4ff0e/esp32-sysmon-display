use esp_hal::{delay::{Delay}, gpio::Output};
use log::info;

pub fn single_beep(beeper:&mut Output<'_>, init_beep: &mut bool){
match init_beep{
    true => return,
    false => {
        // dual beep
        let delay = Delay::new();
        info!("beeped");
        beeper.set_high();
        delay.delay_millis(100);
        beeper.set_low();

        delay.delay_millis(50);
        
        beeper.set_high();
        delay.delay_millis(100);
        beeper.set_low();
        *init_beep = true
    }
}
}