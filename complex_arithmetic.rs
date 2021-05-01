use std::ops::{Add, Mul, Div};
fn main() {
    let n: i32 = 3;
    let result: i32 = imaginary_power(n);
    println!("{}th power of i is {}", n, result);
    let x: Complex = Complex {
        real: 3.0,
        imag: -1.0
    };
    let y: Complex = Complex {
        real: -1.0,
        imag: 2.0,
    };
    let z: Complex = complex_add(&x, &y);
    let m: Complex = complex_mult(&x, &y);
    println!("x = {}", x.str());
    println!("y = {}", y.str());
    println!("add result = {} + {}i", z.real, z.imag);
    println!("mutl result = {} + {}i", m.real, m.imag);
    println!("x + y = {}", (x + y).str());

    let p: Complex = Complex {
        real: 3.0,
        imag: -1.0
    };
    let p_conjugate: Complex = p.conjugate();
    println!("p * p.conjugate() = {}", complex_mult(&p, &p_conjugate).str());

}

fn imaginary_power(n: i32) -> i32 {
    if n % 4 == 0 {
        return 1;
    } else {
        return -1;
    }
}

struct Complex {
    pub real: f32,
    pub imag: f32,
}


struct ComplexInPolar {
    pub r: f32,
    pub theta: f32,
}

impl ComplexInPolar {
    pub fn to_cartesian(&self) -> Complex {
        return Complex {
            real: self.r * self.theta.cos(),
            imag: self.r * self.theta.sin(),
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self {
            real: self.real + other.real, 
            imag: self.imag + other.imag,
        }
    }
}
impl Complex {
    pub fn str(&self) -> String {
        return format!("{} + {}i", self.real, self.imag);
    }

    pub fn conjugate(&self) -> Complex {
        return Complex {
            real: self.real,
            imag: 0.0 - self.imag,
        };
    }

    pub fn mult_conjugate(&self) -> f32 {
        return self.real * self.real + self.imag * self.imag;
    }

    pub fn div_real(&self, real_x: f32) -> Complex {
        return Complex {
            real: self.real / real_x,
            imag: self.imag / real_x,
        }
    }

    pub fn modulus(&self) -> f32 {
        return (self.real * self.real + self.imag * self.imag).sqrt();
    }

    pub fn to_polar(&self) -> ComplexInPolar {
        return ComplexInPolar {
            r: (self.real.powi(2) + self.imag.powi(2)).sqrt(),
            theta: (self.imag / self.real).atan(),
        };
    }
}

fn complex_add(x: &Complex, y: &Complex) -> Complex {
    return Complex {
        real: x.real + y.real, 
        imag: x.imag + y.imag,
    }
}

fn complex_mult(x: &Complex, y: &Complex) -> Complex {
    return Complex {
        real: x.real * y.real - x.imag * y.imag,
        imag: x.imag * y.real + x.real * y.imag,
    }
}

fn complex_div(x: &Complex, y: &Complex) -> Complex {
    let denominator: f32 = y.mult_conjugate();
    return Complex {
        real: (x.real * y.real + x.imag * y.imag)  / denominator,
        imag: (x.imag * y.real - x.real * y.imag) / denominator,
    };
}

fn complex_exp(x: &Complex) -> Complex {
    let exp_real: f32 = x.real.exp();
    return Complex {
        real: exp_real * x.imag.cos(),
        imag: exp_real * x.imag.sin(),
    } 
}


fn complex_exp_real(r: f32, x:Complex) -> Complex {
    let imag_ln_r: f32 = x.imag * r.ln();
    let r_exp_real: f32 = r.powf(x.real);
    return Complex {
        real: r_exp_real * imag_ln_r.cos(),
        imag: r_exp_real * imag_ln_r.sin(),
    }
}

fn polar_mult(x: &ComplexInPolar, y: &ComplexInPolar) -> ComplexInPolar {
    let mut theta: f32 = x.theta + y.theta;
    const PI: f32 = std::f32::consts::PI;
    if theta < 0.0 - PI {
        theta = theta + 2.0 * PI;
    } else if theta > PI {
        theta = theta - 2.0 * PI;
    } else {
        theta = theta;
    }
    return ComplexInPolar {
        r: x.r * y.r,
        theta: theta,
    }
}


fn complex_exp_arbitrary(x: &Complex, y: &Complex) -> Complex {
    let x_in_polar: ComplexInPolar = x.to_polar();
    if x_in_polar.r == 0.0 {
        return Complex {
            real: 0.0,
            imag: 0.0,
        }
    }
    let exp_common = (y.real * x_in_polar.r.ln() - y.imag * x_in_polar.theta).exp();
    let inner_common = y.real * x_in_polar.theta + y.imag * x_in_polar.r.ln();
    let real: f32 = exp_common * inner_common.cos();
    let imag: f32 = inner_common.sin();
    return Complex {
        real: real,
        imag: imag,
    };
}