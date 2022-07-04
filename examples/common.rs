
pub struct Data1 {}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(unused)]
impl Data1 {
    // number of assets
    pub const n  : i32 = 8;
    // Initial uninvested wealth
    pub const w  : f64   = 1.0;
    // Vector of expected returns
    pub const mu : &'static[f64] = &[0.07197, 0.15518, 0.17535, 0.08981, 0.42896, 0.39292, 0.32171, 0.18379];
    // Vector of initial investments
    pub const x0 : &'static[f64] = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Covariance factor
    pub const GT : &'static[f64]   = &[0.30758, 0.12146, 0.11341, 0.11327, 0.17625, 0.11973, 0.10435, 0.10638,
                                       0.     , 0.25042, 0.09946, 0.09164, 0.06692, 0.08706, 0.09173, 0.08506,
                                       0.     , 0.     , 0.19914, 0.05867, 0.06453, 0.07367, 0.06468, 0.01914,
                                       0.     , 0.     , 0.     , 0.20876, 0.04933, 0.03651, 0.09381, 0.07742,
                                       0.     , 0.     , 0.     , 0.     , 0.36096, 0.12574, 0.10157, 0.0571 ,
                                       0.     , 0.     , 0.     , 0.     , 0.     , 0.21552, 0.05663, 0.06187,
                                       0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.22514, 0.03327,
                                       0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.2202 ];
    // risk bound
    pub const gamma : f64 = 0.36;
    pub const m  : &'static [f64]          = &[0.01,  0.01,  0.01,  0.01,   0.01,  0.01,  0.01,  0.01];
    pub const fixed_tcost : &'static [f64] = &[0.01,  0.01,  0.01,  0.01,   0.01,  0.01,  0.01,  0.01];
    pub const prop_tcost  : &'static [f64] = &[0.001, 0.001, 0.001, 0.001,  0.001, 0.001, 0.001, 0.001];
}

pub struct Data2 {
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(unused)]
impl Data2 {
    pub const n     : i32  = 3;
    pub const w     : f64 = 1.0;
    pub const x0    : &'static [f64] = &[0.0, 0.0, 0.0];
    pub const gamma : f64  = 0.05;
    pub const mu    : &'static [f64] = &[0.1073,  0.0737,  0.0627];
    pub const GT    : &'static [f64] = &[0.1667,  0.0232,  0.0013,
                                         0.0000,  0.1033, -0.0022,
                                         0.0000,  0.0000,  0.0338 ];
    pub const fixed_tcost : &'static [f64] = &[0.01,   0.01,  0.01];
    pub const prop_tcost  : &'static [f64] = &[0.001, 0.001, 0.001];
}
