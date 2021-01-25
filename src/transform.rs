//! Fast Fourier Transforms

use core::convert::TryInto;
use core::fmt::Debug;
use core::mem::MaybeUninit;
use core::u16;

use fixed::types::{I1F15, I1F31};
use num_complex::{Complex, Complex32};

use crate::{Error, Result, StatusCode};

/// FFT directions
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    /// Forward FFT (time->frequency)
    Forward = 0,
    /// Inverse FFT (frequency->time)
    Inverse = 1,
}

/// FFT output ordering
#[derive(Debug, Copy, Clone)]
pub enum OutputOrder {
    /// The output is straight out of the Cooley-Tukey algorithm, not in the expected order.
    /// No bit reversal has been applied.
    Raw = 0,
    /// Bit reversal has been applied to the output, leaving the bins in the standard DFT order
    Standard = 1,
}

impl Default for OutputOrder {
    /// Returns the Standard output order
    fn default() -> Self {
        OutputOrder::Standard
    }
}

/// Runs an FFT on floating-point real numbers
pub struct FloatRealFft(cmsis_dsp_sys::arm_rfft_fast_instance_f32);

unsafe impl Send for FloatRealFft {}

impl FloatRealFft {
    /// Initializes an FFT with the specified size
    ///
    /// Valid size values are 32, 64, 128, 256, 512, 1024, 2048, and 4096. This function returns
    /// an error if the size value is not valid.
    pub fn new(size: u16) -> Result<Self> {
        let mut data = MaybeUninit::<cmsis_dsp_sys::arm_rfft_fast_instance_f32>::uninit();
        unsafe {
            cmsis_dsp_sys::arm_rfft_fast_init_f32(data.as_mut_ptr(), size).check_status()?;
            Ok(FloatRealFft(data.assume_init()))
        }
    }

    /// Runs a forward FFT on a set of values, placing the results in output
    ///
    /// # Panics
    ///
    /// This function panics if input or output has a length not equal to the size of this FFT.
    pub fn run(&self, input: &[f32], output: &mut [f32]) {
        self.run_inner(input, output, Direction::Forward);
    }
    /// Runs an inverse FFT on a set of values, placing the results in output
    ///
    /// # Panics
    ///
    /// This function panics if input or output has a length not equal to the size of this FFT.
    pub fn run_inverse(&self, input: &[f32], output: &mut [f32]) {
        self.run_inner(input, output, Direction::Inverse);
    }

    fn run_inner(&self, input: &[f32], output: &mut [f32], direction: Direction) {
        // Check length
        check_fft_size(self.0.fftLenRFFT, input.len());
        check_fft_size(self.0.fftLenRFFT, output.len());

        unsafe {
            cmsis_dsp_sys::arm_rfft_fast_f32(
                &self.0 as *const _ as *mut _,
                input.as_ptr() as *mut _,
                output.as_mut_ptr(),
                direction as _,
            );
        }
    }
}

/// Runs an FFT on Q1.15 fixed-point real numbers
pub struct Q15RealFft(cmsis_dsp_sys::arm_rfft_instance_q15);

unsafe impl Send for Q15RealFft {}

impl Q15RealFft {
    /// Initializes an FFT with the specified size
    ///
    /// Valid size values are 32, 64, 128, 256, 512, 1024, 2048, and 4096. This function returns
    /// an error if the size value is not valid.
    pub fn new(size: u32, direction: Direction, output_order: OutputOrder) -> Result<Self> {
        let mut data = MaybeUninit::<cmsis_dsp_sys::arm_rfft_instance_q15>::uninit();
        unsafe {
            cmsis_dsp_sys::arm_rfft_init_q15(
                data.as_mut_ptr(),
                size,
                direction as _,
                output_order as _,
            )
            .check_status()?;
            Ok(Q15RealFft(data.assume_init()))
        }
    }

    /// Runs an FFT on fixed-point values
    ///
    /// The output type depends on the size of the FFT. To determine how to interpret the output
    /// bits, refer to the table in the arm_rfft_q15 function documentation
    /// at https://www.keil.com/pack/doc/cmsis/DSP/html/group__RealFFT.html#ga00e615f5db21736ad5b27fb6146f3fc5 .
    pub fn run(&self, input: &[I1F15], output: &mut [i16]) {
        check_fft_size(self.0.fftLenReal, input.len());
        check_fft_size(self.0.fftLenReal, output.len());

        unsafe {
            cmsis_dsp_sys::arm_rfft_q15(&self.0, input.as_ptr() as *mut _, output.as_mut_ptr());
        }
    }
}
/// Runs an FFT on Q1.31 fixed-point real numbers
pub struct Q31RealFft(cmsis_dsp_sys::arm_rfft_instance_q31);

unsafe impl Send for Q31RealFft {}

impl Q31RealFft {
    /// Initializes an FFT with the specified size
    ///
    /// Valid size values are 32, 64, 128, 256, 512, 1024, 2048, and 4096. This function returns
    /// an error if the size value is not valid.
    pub fn new(size: u32, direction: Direction, output_order: OutputOrder) -> Result<Self> {
        let mut data = MaybeUninit::<cmsis_dsp_sys::arm_rfft_instance_q31>::uninit();
        unsafe {
            cmsis_dsp_sys::arm_rfft_init_q31(
                data.as_mut_ptr(),
                size,
                direction as _,
                output_order as _,
            )
            .check_status()?;
            Ok(Q31RealFft(data.assume_init()))
        }
    }

    /// Runs an FFT on fixed-point values
    ///
    /// The output type depends on the size of the FFT. To determine how to interpret the output
    /// bits, refer to the table in the arm_rfft_q31 function documentation
    /// at https://www.keil.com/pack/doc/cmsis/DSP/html/group__RealFFT.html#gabaeab5646aeea9844e6d42ca8c73fe3a .
    pub fn run(&self, input: &[I1F31], output: &mut [i32]) {
        check_fft_size(self.0.fftLenReal, input.len());
        check_fft_size(self.0.fftLenReal, output.len());

        unsafe {
            cmsis_dsp_sys::arm_rfft_q31(&self.0, input.as_ptr() as *mut _, output.as_mut_ptr());
        }
    }
}

/// Runs an FFT on floating-point complex numbers
pub struct FloatFft {
    /// Data used by the CMSIS-DSP code
    instance: *const cmsis_dsp_sys::arm_cfft_instance_f32,
}

unsafe impl Send for FloatFft {}

impl FloatFft {
    /// Initializes an FFT with the specified size
    ///
    /// Valid size values are 32, 64, 128, 256, 512, 1024, 2048, and 4096. This function returns
    /// an error if the size value is not valid.
    pub fn new(size: u16) -> Result<Self> {
        let instance = unsafe {
            match size {
                16 => &cmsis_dsp_sys::arm_cfft_sR_f32_len16,
                32 => &cmsis_dsp_sys::arm_cfft_sR_f32_len32,
                64 => &cmsis_dsp_sys::arm_cfft_sR_f32_len64,
                128 => &cmsis_dsp_sys::arm_cfft_sR_f32_len128,
                256 => &cmsis_dsp_sys::arm_cfft_sR_f32_len256,
                512 => &cmsis_dsp_sys::arm_cfft_sR_f32_len512,
                1024 => &cmsis_dsp_sys::arm_cfft_sR_f32_len1024,
                2048 => &cmsis_dsp_sys::arm_cfft_sR_f32_len2048,
                4096 => &cmsis_dsp_sys::arm_cfft_sR_f32_len4096,
                _ => return Err(Error::Argument),
            }
        };
        Ok(FloatFft { instance })
    }

    /// Runs the FFT in-place on a buffer of values
    pub fn run(&self, data: &mut [Complex32], direction: Direction, output_order: OutputOrder) {
        unsafe {
            // FFT size is number of complex values. arm_cfft_f32 expects size * 2 float values.
            // Complex<f32> is layout-compatible.
            check_fft_size((*self.instance).fftLen, data.len());
            cmsis_dsp_sys::arm_cfft_f32(
                self.instance,
                data.as_mut_ptr() as *mut _,
                direction as _,
                output_order as _,
            );
        }
    }
}

/// Runs a 128-bin FFT on floating-point data
///
/// This can offer slightly better performance than FloatFft because it skips the data
/// length check.
pub fn float_fft_128(data: &mut [Complex32; 128], direction: Direction, output_order: OutputOrder) {
    unsafe {
        cmsis_dsp_sys::arm_cfft_f32(
            &cmsis_dsp_sys::arm_cfft_sR_f32_len128,
            data.as_mut_ptr() as *mut f32,
            direction as _,
            output_order as _,
        );
    }
}

/// Runs an FFT on a buffer of samples with a size known at compile time
pub fn fft<D>(data: &mut D, direction: Direction, output_order: OutputOrder)
where
    D: FftBuffer,
{
    data.run_fft(direction, output_order)
}

/// A fixed-length buffer on which an FFT can run
pub trait FftBuffer {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder);
}

impl FftBuffer for [Complex32; 16] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len16,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 32] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len32,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 64] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len64,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 128] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len128,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 256] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len256,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 512] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len512,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 1024] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len1024,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 2048] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len2048,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}
impl FftBuffer for [Complex32; 4096] {
    fn run_fft(&mut self, direction: Direction, output_order: OutputOrder) {
        unsafe {
            cmsis_dsp_sys::arm_cfft_f32(
                &cmsis_dsp_sys::arm_cfft_sR_f32_len4096,
                self.as_mut_ptr() as *mut f32,
                direction as _,
                output_order as _,
            );
        }
    }
}

/// Runs an FFT on Q1.15 fixed-point complex numbers
pub struct Q15Fft {
    /// Data used by the CMSIS-DSP code
    instance: *const cmsis_dsp_sys::arm_cfft_instance_q15,
    /// Transform direction
    direction: Direction,
    /// Output order
    output_order: OutputOrder,
}

unsafe impl Send for Q15Fft {}

impl Q15Fft {
    /// Initializes an FFT with the specified size
    ///
    /// Valid size values are 32, 64, 128, 256, 512, 1024, 2048, and 4096. This function returns
    /// an error if the size value is not valid.
    pub fn new(size: u16, direction: Direction, output_order: OutputOrder) -> Result<Self> {
        let instance = unsafe {
            match size {
                16 => &cmsis_dsp_sys::arm_cfft_sR_q15_len16,
                32 => &cmsis_dsp_sys::arm_cfft_sR_q15_len32,
                64 => &cmsis_dsp_sys::arm_cfft_sR_q15_len64,
                128 => &cmsis_dsp_sys::arm_cfft_sR_q15_len128,
                256 => &cmsis_dsp_sys::arm_cfft_sR_q15_len256,
                512 => &cmsis_dsp_sys::arm_cfft_sR_q15_len512,
                1024 => &cmsis_dsp_sys::arm_cfft_sR_q15_len1024,
                2048 => &cmsis_dsp_sys::arm_cfft_sR_q15_len2048,
                4096 => &cmsis_dsp_sys::arm_cfft_sR_q15_len4096,
                _ => return Err(Error::Argument),
            }
        };

        Ok(Q15Fft {
            instance,
            direction,
            output_order,
        })
    }

    /// Runs the FFT in-place on a buffer of values
    pub fn run(&self, data: &mut [Complex<I1F15>]) {
        unsafe {
            // FFT size is number of complex values. arm_cfft_q15 expects size * 2 u16 values.
            // Complex<I1F15> is layout-compatible.
            check_fft_size((*self.instance).fftLen, data.len());
            cmsis_dsp_sys::arm_cfft_q15(
                self.instance,
                data.as_mut_ptr() as *mut _,
                self.direction as _,
                self.output_order as _,
            );
        }
    }
}

/// Runs an FFT on Q1.31 fixed-point complex numbers
pub struct Q31Fft {
    /// Data used by the CMSIS-DSP code
    instance: *const cmsis_dsp_sys::arm_cfft_instance_q31,
}

unsafe impl Send for Q31Fft {}

impl Q31Fft {
    /// Initializes an FFT with the specified size
    ///
    /// Valid size values are 32, 64, 128, 256, 512, 1024, 2048, and 4096. This function returns
    /// an error if the size value is not valid.
    pub fn new(size: u16) -> Result<Self> {
        let instance = unsafe {
            match size {
                16 => &cmsis_dsp_sys::arm_cfft_sR_q31_len16,
                32 => &cmsis_dsp_sys::arm_cfft_sR_q31_len32,
                64 => &cmsis_dsp_sys::arm_cfft_sR_q31_len64,
                128 => &cmsis_dsp_sys::arm_cfft_sR_q31_len128,
                256 => &cmsis_dsp_sys::arm_cfft_sR_q31_len256,
                512 => &cmsis_dsp_sys::arm_cfft_sR_q31_len512,
                1024 => &cmsis_dsp_sys::arm_cfft_sR_q31_len1024,
                2048 => &cmsis_dsp_sys::arm_cfft_sR_q31_len2048,
                4096 => &cmsis_dsp_sys::arm_cfft_sR_q31_len4096,
                _ => return Err(Error::Argument),
            }
        };
        Ok(Q31Fft { instance })
    }

    /// Runs the FFT in-place on a buffer of values
    pub fn run(
        &self,
        data: &mut [Complex<I1F31>],
        direction: Direction,
        output_order: OutputOrder,
    ) {
        unsafe {
            // FFT size is number of complex values. arm_cfft_q31 expects size * 2 u32 values.
            // Complex<I1F31> is layout-compatible.
            check_fft_size((*self.instance).fftLen, data.len());
            cmsis_dsp_sys::arm_cfft_q31(
                self.instance,
                data.as_mut_ptr() as *mut _,
                direction as _,
                output_order as _,
            );
        }
    }
}

/// Checks that an FFT size is equal to the number of values in an input or output slice
fn check_fft_size<N>(size: N, value_count: usize)
where
    usize: TryInto<N>,
    <usize as TryInto<N>>::Error: Debug,
    N: Debug + PartialEq,
{
    let value_count = value_count
        .try_into()
        .expect("Value count too large for FFT size type");
    assert_eq!(size, value_count);
}
