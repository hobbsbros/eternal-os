//! PID Control Library

/// Defines an abstract control variable that can be controlled by PID control
pub struct ControlVariable {
    expected: f32,
    actual: f32,
    error: f32,
    timestep: u16,
    proportion: f32,
    integral: f32,
    derivative: f32,
    kp: f32,
    ki: f32,
    kd: f32,
}

const DEFAULT_KP: f32 = 0.1;
const DEFAULT_KI: f32 = 0.000_001;
const DEFAULT_KD: f32 = 0.000_1;


impl ControlVariable {
    pub fn new(expected: f32, timestep: u16) -> Self {
        ControlVariable {
            expected: expected,
            actual: 0.0f32,
            error: 0.0f32,
            timestep: timestep,
            proportion: 0.0f32,
            integral: 0.0f32,
            derivative: 0.0f32,
            kp: DEFAULT_KP,
            ki: DEFAULT_KI,
            kd: DEFAULT_KD,
        }
    }

    /// Updates the expected value of the control variable.
    pub fn set_expected(&mut self, expected: f32) {
        self.expected = expected;
    }

    /// Sets the proportional gain.
    pub fn set_kp(&mut self, gain: f32) {
        self.kp = gain;
    }

    /// Sets the integral gain.
    pub fn set_ki(&mut self, gain: f32) {
        self.ki = gain;
    }

    /// Sets the derivative gain.
    pub fn set_kd(&mut self, gain: f32) {
        self.kd = gain;
    }

    /// Sets all gains.
    pub fn set_gains(&mut self, kp: f32, ki: f32, kd: f32) {
        self.set_kp(kp);
        self.set_ki(ki);
        self.set_kd(kd);
    }

    /// Updates the error term.
    fn update_error(&mut self) {
        self.error = self.expected - self.actual;
    }

    /// Updates the PID control variable using the current (actual) value of the variable.
    pub fn step(&mut self, actual: f32) {
        let previous = self.error;

        // Set the actual value of the control variable
        self.actual = actual;

        // Update the error term
        self.update_error();

        // Set the proportional term
        self.proportion = self.error as f32;

        // Compute the integral of the error term
        self.integral += (self.error as f32) * (self.timestep as f32);

        // Set the derivative of the error term
        self.derivative = ((self.error - previous) as f32)/(self.timestep as f32);
    }

    /// Returns the PID correction term (the amount by which to adjust the input to correct the output).
    pub fn get_correction(&mut self) -> f32 {
        let correction: f32 = self.kp*self.proportion + self.ki*self.integral + self.kd*self.derivative;
        correction
    }
}