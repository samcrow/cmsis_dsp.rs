#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate num_complex;
extern crate panic_semihosting;
extern crate libm;
use cortex_m_rt::entry;

extern crate cmsis_dsp;
use cmsis_dsp::transform::FloatRealFft;
use num_complex::Complex32;

#[entry]
fn main() -> ! {
    let fft = FloatRealFft::new(32).expect("Failed to create FFT");

    let input_values: [f32; 32] = [
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0,
    ];
    let mut output_values: [f32; 32] = [0.0; 32];

    fft.run(&input_values, &mut output_values);

    // Test the absolute value function
    let mut abs_output: [f32; 32] = [0.0; 32];
    cmsis_dsp::basic::abs_f32(&output_values, &mut abs_output);

    test_complex_magnitude();

    loop {}
}

fn test_complex_magnitude() {
    let complex_values = [
        Complex32::new(1.0, 2.0),
        Complex32::new(3.0, 4.0),
        Complex32::new(5.0, 6.0),
        Complex32::new(7.0, 8.0),
        Complex32::new(1.0, 2.0),
        Complex32::new(3.0, 4.0),
        Complex32::new(5.0, 6.0),
        Complex32::new(7.0, 8.0),
        Complex32::new(1.0, 2.0),
        Complex32::new(3.0, 4.0),
        Complex32::new(5.0, 6.0),
        Complex32::new(7.0, 8.0),
    ];
    let mut magnitudes = [0.0f32; 12];

    cmsis_dsp::complex::complex_magnitude_f32(&complex_values, &mut magnitudes);
    simple_complex_magnitude(&complex_values, &mut magnitudes);
}

fn simple_complex_magnitude(source: &[Complex32], destination: &mut [f32]) {
    assert_eq!(source.len(), destination.len());
    for (complex, magnitude) in source.iter().zip(destination.iter_mut()) {
        *magnitude = complex.norm();
    }
}
