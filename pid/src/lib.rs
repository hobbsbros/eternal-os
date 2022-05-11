//! PID Control Library

#![no_std]

/// Defines an abstract control variable that can be controlled by PID control
pub struct ControlVariable {
    expected: i16,
    actual: i16,
    error: i16,
    timestep: u8,
    proportion: f32,
    integral: f32,
    derivative: f32,
    kp: f32,
    ki: f32,
    kd: f32,
}

const DEFAULT_KP: f32 = 1.0;
const DEFAULT_KI: f32 = 1.0;
const DEFAULT_KD: f32 = 1.0;


impl ControlVariable {
    fn new(&mut self, expected: i16, timestep: u8) -> Self {
        ControlVariable {
            expected: expected,
            actual: 0i16,
            error: 0i16,
            timestep: timestep,
            proportion: 0.0f32,
            integral: 0.0f32,
            derivative: 0.0f32,
            kp: DEFAULT_KP,
            ki: DEFAULT_KI,
            kd: DEFAULT_KD,
        }
    }

    /// Updates the error term
    fn update_error(&mut self) {
        self.error = self.expected - self.actual;
    }

    /// Update the PID control variable using the current (actual) value of the variable
    fn step(&mut self, actual: i16) {
        let previous = self.error;

        // Set the actual value of the control variable
        self.actual = actual;

        // Update the error term
        self.update_error();

        // Set the proportional term
        self.proportion = (self.error as f32)/(self.actual as f32);

        // Compute the integral of the error term
        self.integral += (self.error as f32) * (self.timestep as f32);

        // Set the derivative of the error term
        self.derivative = ((self.error - previous) as f32)/(self.timestep as f32);
    }

    /// Returns the PID correction term (the amount by which to adjust the input to correct the output).
    fn get_correction(&mut self) -> i16 {
        let correction: f32 = self.kp*self.proportion + self.ki*self.integral + self.kd*self.derivative;
        (correction as i16)
    }
}