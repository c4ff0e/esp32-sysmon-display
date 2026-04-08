use esp_hal::{delay::{Delay}, gpio::Output};

pub fn cpu_unsupported_beep(beeper:&mut Output<'_>, delay: &Delay ){
    // 1 short
    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

}

pub fn gpu_unsupported_beep(beeper:&mut Output<'_>, delay: &Delay){
    // 2 short
    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

    delay.delay_millis(50);

    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

}

pub fn all_unsupported_beep(beeper:&mut Output<'_>, delay: &Delay){
    // 3 short
    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

    delay.delay_millis(50);

    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

    delay.delay_millis(50);

    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();
}

pub fn no_metrics_beep(beeper:&mut Output<'_>, delay: &Delay){
    // 4 short
    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

    delay.delay_millis(50);

    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

    delay.delay_millis(50);

    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();

    delay.delay_millis(50);

    beeper.set_high();
    delay.delay_millis(50);
    beeper.set_low();
}