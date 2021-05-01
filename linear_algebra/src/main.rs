use num::complex::Complex32;
fn main() {
    // let v1: Vec<Complex32> = vec![Complex32::new(1.0, 0.0), Complex32::new(2.0, 0.0), Complex32::new(3.0, 0.0)];
    // let v2: Vec<Complex32> = vec![Complex32::new(4.0, 0.0), Complex32::new(5.0, 0.0), Complex32::new(6.0, 0.0)];
    // let vv: Vec<Vec<Complex32>> = vec![v1, v2];
    // let m1: Matrix = Matrix::new_from_vv(vv);
    // let v3 = vec![Complex32::new(1.0, 0.0)];
    // let v4 = vec![Complex32::new(2.0, 0.0)];
    // let v5 = vec![Complex32::new(3.0, 0.0)];
    // let vv2: Vec<Vec<Complex32>> = vec![v3, v4, v5];
    // let m2: Matrix = Matrix::new_from_vv(vv2);
    // let m3: Matrix =  matrix_mult(m1, m2);
    // let m2: Matrix = Matrix::new(2, 2, Complex32::new(1.0, 1.0));
    // let c: Complex32 = Complex32::new(1.0, 1.0);
    // let m_multed: Matrix = scalar_mult(c, m1);
    // println!("{}", m1.str());
    // println!("{}", m2.str());
    // // let m_added: Matrix = matrix_add(m1, m2);
    // let v1: Vec<Complex32> = vec![Complex32::new(4.0, 1.0), Complex32::new(7.0, 2.0)];
    // let v2: Vec<Complex32> = vec![Complex32::new(2.0, 1.0), Complex32::new(6.0, 2.0)];
    // let vv: Vec<Vec<Complex32>> = vec![v1, v2];
    // let m: Matrix = Matrix::new_from_vv(vv);
    // // let m2: Matrix = matrix_inverse_for_2_2(m);
    // let m3: Matrix = ajoint(&m);
    // println!("{}", m3.str());
    // let mut matrix: Matrix = Matrix::new(2, 2, Complex32::new(0.0, 0.0));
    // matrix.set(0, 0, Complex32::new(0.5, 0.0).sqrt());
    // matrix.set(0, 1, Complex32::new(0.5, 0.0).sqrt());
    // matrix.set(1, 0, Complex32::new(0.0, f32::sqrt(0.5)));
    // matrix.set(1, 1, Complex32::new(0.0, -f32::sqrt(0.5)));
    // println!("is_matrix_unitary = {}", is_matrix_unitary(&matrix));
    let v: Matrix = Matrix::new_from_vv(vec![
        vec![Complex32::new(1.0, 0.0), Complex32::new(2.0, 0.0)],
        vec![Complex32::new(3.0, 0.0), Complex32::new(4.0, 0.0)]
        ]
    );
    let w: Matrix =  Matrix::new_from_vv(vec![
        vec![Complex32::new(5.0, 0.0), Complex32::new(6.0, 0.0)],
        vec![Complex32::new(7.0, 0.0), Complex32::new(8.0, 0.0)]
        ]
    );
    //let normalized_v: Matrix =  normalize(v);
    println!("{}", tensor_prod(v, w).str());
}

struct Matrix {
    matrix: Vec<Vec<Complex32>>,
    m: usize,
    n: usize,
}


impl Matrix {
    pub fn new(m: usize, n: usize, placeholder: Complex32) -> Matrix {
        let vv: Vec<Vec<Complex32>> = vec![vec![placeholder; n]; m];
        return Matrix {
            matrix: vv,
            m: m,
            n: n,
        };
    }

    pub fn new_from_vv(vv: Vec<Vec<Complex32>>) -> Matrix {
        let m: usize = vv.len();
        if m == 0 {
            return Matrix {
                matrix: vv,
                m: 0,
                n: 0,
            }
        } else {
            let n: usize = (&vv[0]).len();
            return Matrix {
                matrix: vv,
                m: m,
                n: n,
            }
        }
    }
    pub fn str(&self) -> String {
        let mut matrix_str: String = "".to_owned();
        for i in 0..self.m {
            for j in 0..self.n {
                let row_str: String = format!("***[{}, {}] = ({}, {})***", i, j, self.get(i, j).re, self.get(i, j).im);
                matrix_str.push_str(&row_str);
            }
            matrix_str.push_str(&String::from("\n"));
        }
        return matrix_str;
    }
    pub fn get_m(&self) -> usize {
        return self.m;
    }

    pub fn get_n(&self) -> usize {
        return self.n;
    }

    pub fn get(&self, i: usize, j: usize) -> Complex32 {
        return self.matrix[i][j];
    }

    pub fn set(&mut self, i: usize, j: usize, x: Complex32) {
        self.matrix[i][j] = x;
    }

}
fn matrix_add(a: Matrix, b: Matrix) -> Matrix {
    if a.get_m() != b.get_m() || a.get_n() != b.get_n() {
        panic!("matrix_add not supported for not_matched matrixes");
    }
    let m: usize = a.get_m();
    let n: usize = a.get_m();
    let mut added_matrix: Matrix = Matrix::new(m, n, Complex32::new(0.0, 0.0));
    for i in 0..m {
        for j in 0..n {
            let a_item: Complex32 = a.get(i, j);
            let b_item: Complex32 = b.get(i, j);
            let added_item: Complex32 = a_item + b_item;
            added_matrix.set(i, j, added_item);
        }
    }
    return added_matrix;
}


fn scalar_mult(x: Complex32, a: Matrix) -> Matrix {
    let m: usize = a.get_m();
    let n: usize = a.get_n();
    let mut scalar_multed_matrix: Matrix = Matrix::new(m, n, Complex32::new(0.0, 0.0));
    for i in 0..m {
        for j in 0..n {
            let item: Complex32 = a.get(i, j);
            let scalar_multed_item: Complex32 = item * x;
            scalar_multed_matrix.set(i, j, scalar_multed_item);
        }
    }
    return scalar_multed_matrix;
}

fn matrix_mult(a: &Matrix, b: &Matrix) -> Matrix {
    let a_m: usize = a.get_m();
    let a_n: usize = a.get_n();
    let b_m: usize = b.get_m();
    let b_n: usize = b.get_n();
    if a_n != b_m {
        panic!("matrix_mult not supported for not_matched matrixes");
    }
    let mut c = Matrix::new(a_m, b_n, Complex32::new(0.0, 0.0));
    for i in 0..a_m {
        for j in 0..b_n {
            let mut c_i_j: Complex32 = Complex32::new(0.0, 0.0);
            for k in 0..a_n {
                let a_i_k: Complex32 = a.get(i, k);
                let b_k_j: Complex32 = b.get(k, j);
                c_i_j = c_i_j + a_i_k * b_k_j;
            }
            c.set(i, j, c_i_j);
        }
    }
    return c;
}

fn matrix_inverse_for_2_2(matrix: Matrix) -> Matrix {
    let m: usize = matrix.get_m();
    let n: usize = matrix.get_n();
    if m != 2 || n != 2 {
        panic!("matrix inverse only support on 2x2 matrix");
    }
    let a: Complex32 = matrix.get(0, 0);
    let b: Complex32 = matrix.get(0, 1);
    let c: Complex32 = matrix.get(1, 0);
    let d: Complex32 = matrix.get(1, 1);
    let determinant: Complex32 = a * d - b * c;
    let determinant_inv = determinant.inv();
    let matrix_part = Matrix::new_from_vv(vec![vec![d, -b], vec![-c, a]]);
    let inversed_matrix = scalar_mult(determinant_inv, matrix_part);
    return inversed_matrix;
}

fn transpose(matrix: &Matrix) -> Matrix {
    let m: usize = matrix.get_m();
    let n: usize = matrix.get_n();
    if m == 0 || n == 0 {
        return Matrix::new(0, 0, Complex32::new(0.0, 0.0));
    } 
    let mut transposed_matrix: Matrix = Matrix::new(n, m, Complex32::new(0.0, 0.0));
    for i in 0..m {
        for j in 0..n {
            transposed_matrix.set(j, i, matrix.get(i, j));
        }
    }
    return transposed_matrix;
}


fn conjugate(matrix: &Matrix) -> Matrix {
    let m: usize = matrix.get_m();
    let n: usize = matrix.get_n();
    let mut conjugated_matrix: Matrix = Matrix::new(m, n, Complex32::new(0.0, 0.0));
    for i in 0..m {
        for j in 0..n {
            let item_i_j:Complex32 = matrix.get(i, j);
            let item_i_j_conjugated = item_i_j.conj();
            conjugated_matrix.set(i, j, item_i_j_conjugated);
        }
    }
    return conjugated_matrix;
}

fn ajoint(matrix: &Matrix) -> Matrix {
    return transpose(&conjugate(matrix));
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let m_self: usize = self.get_m();
        let n_self: usize = self.get_n();
        let m_other: usize = other.get_m();
        let n_other: usize = other.get_n();
        println!("1");
        if m_self != m_other || n_self != n_other {
            return false;
        }
        println!("2");
        for i in 0..m_self {
            for j in 0..n_self {
                let self_i_j: Complex32  = self.get(i, j);
                let other_i_j: Complex32 = other.get(i, j);
                if !approx_equal(self_i_j.re, other_i_j.re, 2) {
                    println!("self_i_j = {} ** other_i_j = {}", self_i_j.re, other_i_j.re);
                    return false;
                }
            }
        }
        return true;
    }
}

fn is_matrix_unitary(matrix: &Matrix) -> bool {
    let m: usize = matrix.get_m();
    let n: usize = matrix.get_n();
    if m != n {
        panic!("is_matrix_unitary not supported for not nxn matrix.");
    }
    println!("matrix = \n {}", matrix.str());
    let adjointed_matrix: Matrix = ajoint(matrix);
    println!("adjointed_matrix = \n {}", adjointed_matrix.str());
    let identity_matrix: Matrix = get_identity_matrix(n);
    println!("identity_matrix = \n {}", identity_matrix.str()); 
    let origin_mult_adjointed: Matrix = matrix_mult(matrix, &adjointed_matrix);
    println!("origin_mult_adjointed = \n {}", origin_mult_adjointed.str()); 
    let adjointed_mult_origin: Matrix = matrix_mult(&adjointed_matrix, matrix);
    println!("adjointed_mult_origin = \n {}", adjointed_mult_origin.str()); 
    if origin_mult_adjointed == identity_matrix && adjointed_mult_origin == identity_matrix {
        return true;
    } else {
        return false;
    }

}

fn get_identity_matrix(n: usize) -> Matrix {
    let mut i_matrix: Matrix = Matrix::new(n, n, Complex32::new(0.0, 0.0));
    for i in 0..n {
        i_matrix.set(i, i, Complex32::new(1.0, 0.0));
    }
    return i_matrix;
}

fn inner_prod(v: &Matrix, w: &Matrix) -> Complex32 {
    let m_v: usize = v.get_m();
    let n_v: usize = v.get_n();
    let m_w: usize = w.get_m();
    let n_w: usize = w.get_n();

    if m_v < 1 || n_v != 1 || m_w < 1 || n_w != 1 || m_v != m_w {
        panic!("inner_prod only supported for nx1 and nx1 matrixes, not for {}x{} and {}x{}", m_v, n_v, m_w, n_w);
    }
    let v_adjointed: Matrix = ajoint(&v);
    return matrix_mult(&v_adjointed, &w).get(0, 0);
}


fn approx_equal (a: f32,b: f32,dp: u8) -> bool {
    let p:f32 = 10f32.powf(-(dp as f32));
    if (a-b).abs() < p {
        return true;
    } else {
        return false;
    }
}

fn normalize(v: Matrix) -> Matrix {
    let norm: Complex32 = inner_prod(&v, &v).sqrt();
    return scalar_mult(norm.inv(), v);
}

fn outer_prod(v: &Matrix, w: &Matrix) -> Matrix {
    let m_v: usize = v.get_m();
    let n_v: usize = v.get_n();
    let m_w: usize = w.get_m();
    let n_w: usize = w.get_n();
    if m_v < 1 || n_v != 1 || m_w < 1 || n_w != 1 {
        panic!("outer_prod only supported for nx1 and m*1 matrixes, not for {}x{} and {}x{}", m_v, n_v, m_w, n_w);
    }
    let mut r: Matrix = Matrix::new(m_v, m_w, Complex32::new(0.0, 0.0));
    for i in 0..m_v {
        for j in 0..m_w {
            let v_i: Complex32 = v.get(i, 0);
            let w_j: Complex32 = w.get(j, 0);
            println!("v_i = {}, w_j = {}, v_i * w_j.conj() = {}", v_i, w_j, v_i * w_j.conj());
            r.set(i, j, v_i * w_j.conj());
        }
    } 
    return r;
}

fn tensor_prod(a: Matrix, b: Matrix) -> Matrix {
    let m_a: usize = a.get_m();
    let n_a: usize = a.get_n();
    let m_b: usize = b.get_m();
    let n_b: usize = b.get_n();
    if m_a == 0 || n_a == 0 || m_b == 0 || n_b == 0 {
        return Matrix::new(0, 0, Complex32::new(0.0, 0.0));
    }
    let mut r: Matrix = Matrix::new(m_a * m_b, n_a * n_b, Complex32::new(0.0, 0.0));
    for i_a in 0..m_a {
        for j_a in 0..n_a {
            for i_b in 0..m_b {
                for j_b in 0..n_b {
                    let i_r: usize = i_a * n_b + i_b;
                    let j_r: usize = j_a * m_b + j_b;
                    let r_i_j: Complex32 = a.get(i_a, j_a) * b.get(i_b, j_b);
                    r.set(i_r, j_r, r_i_j);
                }
            }
        }
    }
    return r;
}


#[test]
fn is_matrix_unitary_test() {
    let mut matrix: Matrix = Matrix::new(2, 2, Complex32::new(0.0, 0.0));
    matrix.set(0, 0, Complex32::new(0.5, 0.0).sqrt());
    matrix.set(0, 1, Complex32::new(0.5, 0.0).sqrt());
    matrix.set(1, 0, Complex32::new(0.0, f32::sqrt(0.5)));
    matrix.set(1, 1, Complex32::new(0.0, -f32::sqrt(0.5)));
    //println!("is_matrix_unitary = {}",is_matrix_unitary(&matrix));
    assert!(is_matrix_unitary(&matrix) == true);
}


#[test] 
fn inner_prod_test() {
    let v: Matrix = transpose(&Matrix::new_from_vv(vec![vec![Complex32::new(-6.0, 0.0), Complex32::new(0.0, 9.0)]]));
    let w: Matrix = transpose(&Matrix::new_from_vv(vec![vec![Complex32::new(3.0, 0.0), Complex32::new(-8.0, 0.0)]]));
    let inner_prod: Complex32 = inner_prod(&v, &w);
    println!("{}", inner_prod);
    assert!(inner_prod == Complex32::new(-18.0, 72.0));
}


#[test]
fn normalize_test() {
    let v: Matrix = transpose(&Matrix::new_from_vv(vec![vec![Complex32::new(3.0, 0.0), Complex32::new(-8.0, 0.0)]]));
    println!("{}", normalize(v));
}