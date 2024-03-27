impl std::ops::Add for ComplexNumbers {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl std::ops::Add<&ComplexNumbers> for &ComplexNumbers {
    type Output = ComplexNumbers;

    fn add(self, other: &ComplexNumbers) -> Self::Output {
        Self::Output {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl<T> Add<T> for ComplexNumbers
where
    T: Add<Output = T> + Into<f64> + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self::Output {
            real: self.real + other.into(),
            imaginary: self.imaginary,
        }
    }
}

impl std::ops::Sub for ComplexNumbers {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl std::ops::Sub<&ComplexNumbers> for &ComplexNumbers {
    type Output = ComplexNumbers;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl<T> std::ops::Sub<T> for ComplexNumbers
where
    T: std::ops::Sub<Output = T> + Into<f64> + Copy,
{
    type Output = ComplexNumbers;

    fn sub(self, other: T) -> Self::Output {
        Self::Output {
            real: self.real - other,
            imaginary: self.imaginary,
        }
    }
}

impl std::ops::Mul for ComplexNumbers {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl std::ops::Div for ComplexNumbers {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator = other.real.powi(2) + other.imaginary.powi(2);
        Self {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denominator,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denominator,
        }
    }
}