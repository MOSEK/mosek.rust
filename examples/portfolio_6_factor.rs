
//!
//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! File : portfolio_6_factor.rs
//!
//! Purpose :   Implements a portfolio optimization model using factor model.
//!

extern crate mosek;
extern crate itertools;
use mosek::{Task,Boundkey,Objsense,Streamtype,Soltype,Transpose,Solsta};
use itertools::{iproduct};
use std::convert::TryInto;


const INF : f64 = 0.0;

#[allow(non_snake_case)]
fn portfolio(w      : f64,
             mu     : &[f64],
             x0     : &[f64],
             gammas : &[f64],
             theta  : &[f64],
             GT     : &Matrix) -> Result<Vec<(f64,f64)>,String> {

    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    let (kx,nx) = GT.size();
    if mu.len() != nx { panic!("Mismatching data"); }

    let k : i32 = kx.try_into().unwrap();
    let n : i32 = nx.try_into().unwrap();
    let total_budget : f64 = w + x0.iter().sum::<f64>();

    //Offset of variables into the API variable.
    let numvar = n;
    let voff_x : i32 = 0;

    // Constraint offset
    let coff_bud : i32 = 0;

    // Holding variable x of length n
    // No other auxiliary variables are needed in this formulation
    task.append_vars(numvar)?;

    // Setting up variable x 
    for j in 0..n {
        task.put_var_name(voff_x+j,format!("x[{}]",j+1).as_str())?;
    }
    task.put_var_bound_slice_const(voff_x,voff_x+n, Boundkey::LO,0.0, INF)?;

    // One linear constraint: total budget
    task.append_cons(1)?;
    task.put_con_name(coff_bud, "budget")?;

    /* Coefficients in the first row of A */
    for j in 0..n {
        task.put_aij(coff_bud,voff_x + j, 1.0)?;
    }
    task.put_con_bound(coff_bud, Boundkey::FX, total_budget, total_budget)?;

    // Input (gamma, G_factor_T x, diag(sqrt(theta))*x) in the AFE (affine expression) storage
    // We need k+n+1 rows and we fill them in in three parts
    task.append_afes((k+n) as i64 + 1)?;
    // 1. The first affine expression = gamma, will be specified later
    // 2. The next k expressions comprise G_factor_T*x, we add them row by row
    //    transposing the matrix G_factor on the fly
    task.put_afe_f_row_list((1..1+k as i64).collect::<Vec<i64>>().as_slice(), // f row idxs
                            vec![n; k as usize].as_slice(), // row lengths
                            (0..GT.len() as i64).step_by(n as usize).collect::<Vec<i64>>().as_slice(), // row ptr
                            iproduct!(0..k,0..n).map(|(_,b)| b).collect::<Vec<i32>>().as_slice(), // varidx, 0..n repeated k times
                            GT.data_by_row().as_slice())?;
    // 3. The remaining n rows contain sqrt(theta) on the diagonal
    for (i,thetai) in (0..n).zip(theta.iter()) {
        task.put_afe_f_entry(i as i64 + 1 + k as i64, voff_x + i, thetai.sqrt())?;
    }

    // Input the affine conic constraint (gamma, GT*x) \in QCone
    // Add the quadratic domain of dimension k+1
    let qdom = task.append_quadratic_cone_domain(k as i64+ 1)?;
    // Add the constraint
    task.append_acc_seq(qdom, 0, vec![0.0; k as usize+1].as_slice())?;
    task.put_acc_name(0, "risk")?;

    // Objective: maximize expected return mu^T x
    for (j,&muj) in (0..n).zip(mu.iter()) {
        task.put_c_j(voff_x + j, muj)?;
    }
    task.put_obj_sense(Objsense::MAXIMIZE)?;

    Ok(gammas.iter().filter_map(|&gamma| {
        // Specify gamma in ACC
        
        task.put_afe_g(0, gamma).ok()?;
        task.optimize().ok()?;
        /* Display solution summary for quick inspection of results */
        let _ = task.solution_summary(Streamtype::LOG);
        let _ = task.write_data(format!("portfolio_6_factor-{}.ptf",gamma).as_str());

        // Check if the interior point solution is an optimal point
        if task.get_sol_sta(Soltype::ITR).ok()? != Solsta::OPTIMAL {
            // See https://docs.mosek.com/latest/rustapi/accessing-solution.html about handling solution statuses.
            eprintln!("Solution not optimal!");
            std::process::exit(1);
        }

        /* Read the results */
        let mut xx = vec![0.0; n as usize];
        task.get_xx_slice(Soltype::ITR, voff_x, voff_x + n, xx.as_mut_slice()).ok()?;
        Some((gamma,xx.iter().zip(mu.iter()).map(|(&xj,&muj)| xj*muj).sum::<f64>()))
    }).collect::<Vec<(f64,f64)>>())
}



#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    // Since the value infinity is never used, we define
    // 'infinity' for symbolic purposes only
    let n : i32      = 8;
    let w  = 1.0;
    let mu = &[0.07197, 0.15518, 0.17535, 0.08981, 0.42896, 0.39292, 0.32171, 0.18379];
    let x0 = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Factor exposure matrix, n x 2
    let B = Matrix::new_by_row(n as usize, 2,
                               [ 0.4256,  0.1869,
                                 0.2413,  0.3877,
                                 0.2235,  0.3697,
                                 0.1503,  0.4612,
                                 1.5325, -0.2633,
                                 1.2741, -0.2613,
                                 0.6939,  0.2372,
                                 0.5425,  0.2116 ].to_vec()).unwrap();

    // Factor covariance matrix, 2x2
    let S_F = Matrix::new_by_row(2,2,
                                 [ 0.0620, 0.0577,
                                   0.0577, 0.0908 ].to_vec()).unwrap();

    // Specific risk components
    let theta : &[f64] = &[0.0720, 0.0508, 0.0377, 0.0394, 0.0663, 0.0224, 0.0417, 0.0459];
    let S_sqrt_theta = Matrix::diag_matrix(theta.iter().map(|&v| v.sqrt()).collect());
    let P = S_F.cholesky().unwrap();
    let BP = B.mul(&P).unwrap();

    //let GT  = BP.concat_h(&S_theta.sqrt_element().unwrap()).unwrap().transpose();
    let GT  = BP.concat_h(&S_sqrt_theta).unwrap().transpose();
    let gammas = &[0.24, 0.28, 0.32, 0.36, 0.4, 0.44, 0.48];

    for (gamma,expret) in portfolio(w,
                                    mu,
                                    x0,
                                    gammas,
                                    theta,
                                    &GT)? {
        println!("Expected return {:.4e} for gamma {:.2}", expret, gamma);
    }
    Ok(())
}

// Matrix with data stored in colunn format
#[derive(Copy,Clone)]
pub enum MatrixOrder {
    ByRow,
    ByCol
}

pub struct Matrix {
    fmt  : MatrixOrder,
    dimi : usize,
    dimj : usize,
    data : Vec<f64>
}

impl Matrix {
    pub fn new(fmt : MatrixOrder, dimi : usize, dimj : usize, data : Vec<f64>) -> Option<Matrix> {
        if dimi*dimj == data.len() {
            Some(Matrix{fmt,dimi,dimj,data})
        }
        else {
            None
        }
    }

    pub fn new_by_col(dimi : usize, dimj : usize, data : Vec<f64>) -> Option<Matrix> {
        Matrix::new(MatrixOrder::ByCol,dimi,dimj,data)
    }
    pub fn new_by_row(dimi : usize, dimj : usize, data : Vec<f64>) -> Option<Matrix> {
        Matrix::new(MatrixOrder::ByRow,dimi,dimj,data)
    }

    pub fn size(&self) -> (usize,usize) { (self.dimi,self.dimj) }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn diag_matrix(data : Vec<f64>) -> Matrix {
        let mut rdata = vec![0.0; data.len()*data.len()];
        for (r,&v) in rdata.iter_mut().step_by(data.len()+1).zip(data.iter()) { *r = v; }
        Matrix{fmt:MatrixOrder::ByCol,dimi:data.len(),dimj:data.len(),data:rdata}
    }
    pub fn transpose(&self) -> Matrix {
        Matrix {
            fmt : match self.fmt { MatrixOrder::ByRow => MatrixOrder::ByCol, MatrixOrder::ByCol => MatrixOrder::ByRow },
            dimi : self.dimj,
            dimj : self.dimi,
            data : self.data.as_slice().to_vec()
        }
    }

    pub fn data_by_row(&self) -> Vec<f64> {
        match self.fmt {
            MatrixOrder::ByRow => self.data.clone(),
            MatrixOrder::ByCol => iproduct!(0..self.dimi,0..self.dimj).map(|(i,j)| unsafe { *self.data.get_unchecked(self.dimi*j+i) }).collect()
        }
    }

    pub fn data_by_col(&self) -> Vec<f64> {
        match self.fmt {
            MatrixOrder::ByCol => self.data.clone(),
            MatrixOrder::ByRow => iproduct!(0..self.dimj,0..self.dimi).map(|(j,i)| unsafe { *self.data.get_unchecked(self.dimj*i+j) }).collect()
        }
    }

    pub fn data_by_row_to(&self,r : &mut[f64]) {
        match self.fmt {
            MatrixOrder::ByRow => { r.clone_from_slice(self.data.as_slice()); },
            MatrixOrder::ByCol => {
                if r.len() != self.data.len() { panic!("Incorrect result array length"); }
                for (r,(i,j)) in r.iter_mut().zip(iproduct!(0..self.dimi,0..self.dimj)) {
                    *r = unsafe { *self.data.get_unchecked(self.dimi*j+i) }
                }
            }
        }
    }

    pub fn data_by_col_to(&self,r : &mut[f64]) {
        match self.fmt {
            MatrixOrder::ByCol => { r.clone_from_slice(self.data.as_slice()); },
            MatrixOrder::ByRow => {
                for (r,(j,i)) in r.iter_mut().zip(iproduct!(0..self.dimj,0..self.dimi)) {
                    *r = unsafe { *self.data.get_unchecked(self.dimj*i+j) };
                }
            }
        }
    }

    pub fn cholesky(&self) -> Option<Matrix> {
        if self.dimi != self.dimj { None }
        else {
            let mut resdata = self.data_by_col();
            // zero data in the non-tril part
            for ((j,i),d) in iproduct!(0..self.dimj,0..self.dimi).zip(resdata.iter_mut()) { if i < j {*d = 0.0}; }
            if let Ok(_) = mosek::potrf(mosek::Uplo::LO,
                                        self.dimi.try_into().unwrap(),
                                        resdata.as_mut_slice()) {
                Some(Matrix{fmt:MatrixOrder::ByCol,dimi:self.dimi,dimj:self.dimj,data:resdata})
            }
            else { None }
        }
    }

    pub fn mul(&self,other : &Matrix) -> Option<Matrix> {
        if self.dimj != other.dimi { None }
        else {
            let mut resdata = vec![0.0; self.dimi * other.dimj];
            if let Ok(_) = mosek::gemm(match self.fmt { MatrixOrder::ByCol => Transpose::NO, MatrixOrder::ByRow => Transpose::YES },
                                       match other.fmt { MatrixOrder::ByCol => Transpose::NO, MatrixOrder::ByRow => Transpose::YES },
                                       self.dimi.try_into().unwrap(),
                                       other.dimj.try_into().unwrap(),
                                       self.dimj.try_into().unwrap(),
                                       1.0,
                                       self.data.as_slice(),
                                       other.data.as_slice(),
                                       1.0,
                                       resdata.as_mut_slice()) {
                Matrix::new_by_col(self.dimi,other.dimj,resdata)
            }
            else {
                None
            }
        }
    }

    pub fn concat_h(&self, other : & Matrix) -> Option<Matrix> {
        if self.dimi != other.dimi {
            None
        }
        else {
            let mut resdata = vec![0.0; self.data.len() + other.data.len()];

            self.data_by_col_to(& mut resdata[..self.len()]);
            other.data_by_col_to(& mut resdata[self.len()..]);

            Matrix::new_by_col(self.dimi,self.dimj+other.dimj,resdata)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
