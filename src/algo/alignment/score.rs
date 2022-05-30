use lazy_static::lazy_static;

// BLOSUM62 scoring matrix, 
// Characters have been rearranged and padding was introduced for {B, J, O, U, X, Z} to match ASCI encoding
lazy_static! {
    static ref BLOSUM62: ndarray::Array2<i32> = ndarray::Array::from_shape_vec((26, 26), vec![
    /*   A    B*   C    D    E    F    G    H    I    J*   K    L    M    N    O*   P    Q    R    S    T    U*   V    W    X*   Y    Z*      */
         4,   0,   0,  -2,  -1,  -2,   0,  -2,  -1,   0,  -1,  -1,  -1,  -2,   0,  -1,  -1,  -1,   1,   0,   0,   0,  -3,   0,  -2,   0, /* A  */
         0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, /* B* */
         0,   0,   9,  -3,  -4,  -2,  -3,  -3,  -1,   0,  -3,  -1,  -1,  -3,   0,  -3,  -3,  -3,  -1,  -1,   0,  -1,  -2,   0,  -2,   0, /* C  */
        -2,   0,  -3,   6,   2,  -3,  -1,  -1,  -3,   0,  -1,  -4,  -3,   1,   0,  -1,   0,  -2,   0,  -1,   0,  -3,  -4,   0,  -3,   0, /* D  */
        -1,   0,  -4,   2,   5,  -3,  -2,   0,  -3,   0,   1,  -3,  -2,   0,   0,  -1,   2,   0,   0,  -1,   0,  -2,  -3,   0,  -2,   0, /* E  */
        -2,   0,  -2,  -3,  -3,   6,  -3,  -1,   0,   0,  -3,   0,   0,  -3,   0,  -4,  -3,  -3,  -2,  -2,   0,  -1,   1,   0,   3,   0, /* F  */
         0,   0,  -3,  -1,  -2,  -3,   6,  -2,  -4,   0,  -2,  -4,  -3,   0,   0,  -2,  -2,  -2,   0,  -2,   0,  -3,  -2,   0,  -3,   0, /* G  */
        -2,   0,  -3,  -1,   0,  -1,  -2,   8,  -3,   0,  -1,  -3,  -2,   1,   0,  -2,   0,   0,  -1,  -2,   0,  -3,  -2,   0,   2,   0, /* H  */
        -1,   0,  -1,  -3,  -3,   0,  -4,  -3,   4,   0,  -3,   2,   1,  -3,   0,  -3,  -3,  -3,  -2,  -1,   0,   3,  -3,   0,  -1,   0, /* I  */
         0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, /* J* */
        -1,   0,  -3,  -1,   1,  -3,  -2,  -1,  -3,   0,   5,  -2,  -1,   0,   0,  -1,   1,   2,   0,  -1,   0,  -2,  -3,   0,  -2,   0, /* K  */
        -1,   0,  -1,  -4,  -3,   0,  -4,  -3,   2,   0,  -2,   4,   2,  -3,   0,  -3,  -2,  -2,  -2,  -1,   0,   1,  -2,   0,  -1,   0, /* L  */
        -1,   0,  -1,  -3,  -2,   0,  -3,  -2,   1,   0,  -1,   2,   5,  -2,   0,  -2,   0,  -1,  -1,  -1,   0,   1,  -1,   0,  -1,   0, /* M  */
        -2,   0,  -3,   1,   0,  -3,   0,   1,  -3,   0,   0,  -3,  -2,   6,   0,  -2,   0,   0,   1,   0,   0,  -3,  -4,   0,  -2,   0, /* N  */
         0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, /* O* */
        -1,   0,  -3,  -1,  -1,  -4,  -2,  -2,  -3,   0,  -1,  -3,  -2,  -2,   0,   7,  -1,  -2,  -1,  -1,   0,  -2,  -4,   0,  -3,   0, /* P  */
        -1,   0,  -3,   0,   2,  -3,  -2,   0,  -3,   0,   1,  -2,   0,   0,   0,  -1,   5,   1,   0,  -1,   0,  -2,  -2,   0,  -1,   0, /* Q  */
        -1,   0,  -3,  -2,   0,  -3,  -2,   0,  -3,   0,   2,  -2,  -1,   0,   0,  -2,   1,   5,  -1,  -1,   0,  -3,  -3,   0,  -2,   0, /* R  */
         1,   0,  -1,   0,   0,  -2,   0,  -1,  -2,   0,   0,  -2,  -1,   1,   0,  -1,   0,  -1,   4,   1,   0,  -2,  -3,   0,  -2,   0, /* S  */
         0,   0,  -1,  -1,  -1,  -2,  -2,  -2,  -1,   0,  -1,  -1,  -1,   0,   0,  -1,  -1,  -1,   1,   5,   0,   0,  -2,   0,  -2,   0, /* T  */
         0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, /* U* */
         0,   0,  -1,  -3,  -2,  -1,  -3,  -3,   3,   0,  -2,   1,   1,  -3,   0,  -2,  -2,  -3,  -2,   0,   0,   4,  -3,   0,  -1,   0, /* V  */
        -3,   0,  -2,  -4,  -3,   1,  -2,  -2,  -3,   0,  -3,  -2,  -1,  -4,   0,  -4,  -2,  -3,  -3,  -2,   0,  -3,  11,   0,   2,   0, /* W  */
         0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, /* X* */
        -2,   0,  -2,  -3,  -2,   3,  -3,   2,  -1,   0,  -2,  -1,  -1,  -2,   0,  -3,  -1,  -2,  -2,  -2,   0,  -1,   2,   0,   7,   0, /* Y  */
         0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0  /* Z* */
    ]).unwrap();
}

pub fn blosum62(a: &u8, b: &u8) -> i32 {
    // print!("{}   {}\n", a - 65 , b-65);
    BLOSUM62[((*a as usize) - 65, (*b as usize) - 65)]
}


// PAM250 scoring matrix
lazy_static! {
    static ref PAM250: ndarray::Array2<i32> = ndarray::Array::from_shape_vec((26, 26), vec![
         2,  0, -2,  0,  0, -3,  1, -1, -1, -2, -1, -2, -1,  0,  0,  1,  0, -2,  1,  1,  0,  0, -6,  0, -3,  0,  
         0,  3, -4,  3,  3, -4,  0,  1, -2, -3,  1, -3, -2,  2, -1, -1,  1, -1,  0,  0, -1, -2, -5, -1, -3,  2,  
        -2, -4, 12, -5, -5, -4, -3, -3, -2, -4, -5, -6, -5, -4, -3, -3, -5, -4,  0, -2, -3, -2, -8, -3,  0, -5,  
         0,  3, -5,  4,  3, -6,  1,  1, -2, -3,  0, -4, -3,  2, -1, -1,  2, -1,  0,  0, -1, -2, -7, -1, -4,  3,  
         0,  3, -5,  3,  4, -5,  0,  1, -2, -3,  0, -3, -2,  1, -1, -1,  2, -1,  0,  0, -1, -2, -7, -1, -4,  3,  
        -3, -4, -4, -6, -5,  9, -5, -2,  1,  2, -5,  2,  0, -3, -2, -5, -5, -4, -3, -3, -2, -1,  0, -2,  7, -5,  
         1,  0, -3,  1,  0, -5,  5, -2, -3, -4, -2, -4, -3,  0, -1,  0, -1, -3,  1,  0, -1, -1, -7, -1, -5,  0,  
        -1,  1, -3,  1,  1, -2, -2,  6, -2, -2,  0, -2, -2,  2, -1,  0,  3,  2, -1, -1, -1, -2, -3, -1,  0,  2,  
        -1, -2, -2, -2, -2,  1, -3, -2,  5,  4, -2,  2,  2, -2, -1, -2, -2, -2, -1,  0, -1,  4, -5, -1, -1, -2,  
        -2, -3, -4, -3, -3,  2, -4, -2,  4,  4, -3,  4,  3, -3, -1, -3, -2, -3, -2, -1, -1,  3, -4, -1, -1, -3,  
        -1,  1, -5,  0,  0, -5, -2,  0, -2, -3,  5, -3,  0,  1, -1, -1,  1,  3,  0,  0, -1, -2, -3, -1, -4,  0,  
        -2, -3, -6, -4, -3,  2, -4, -2,  2,  4, -3,  6,  4, -3, -1, -3, -2, -3, -3, -2, -1,  2, -2, -1, -1, -3,  
        -1, -2, -5, -3, -2,  0, -3, -2,  2,  3,  0,  4,  6, -2, -1, -2, -1,  0, -2, -1, -1,  2, -4, -1, -2, -2,  
         0,  2, -4,  2,  1, -3,  0,  2, -2, -3,  1, -3, -2,  2,  0,  0,  1,  0,  1,  0,  0, -2, -4,  0, -2,  1,  
         0, -1, -3, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,  0, -1, -1, -1, -1,  0,  0, -1, -1, -4, -1, -2, -1,  
         1, -1, -3, -1, -1, -5,  0,  0, -2, -3, -1, -3, -2,  0, -1,  6,  0,  0,  1,  0, -1, -1, -6, -1, -5,  0,  
         0,  1, -5,  2,  2, -5, -1,  3, -2, -2,  1, -2, -1,  1, -1,  0,  4,  1, -1, -1, -1, -2, -5, -1, -4,  3,  
        -2, -1, -4, -1, -1, -4, -3,  2, -2, -3,  3, -3,  0,  0, -1,  0,  1,  6,  0, -1, -1, -2,  2, -1, -4,  0,  
         1,  0,  0,  0,  0, -3,  1, -1, -1, -2,  0, -3, -2,  1,  0,  1, -1,  0,  2,  1,  0, -1, -2,  0, -3,  0,  
         1,  0, -2,  0,  0, -3,  0, -1,  0, -1,  0, -2, -1,  0,  0,  0, -1, -1,  1,  3,  0,  0, -5,  0, -3, -1,  
         0, -1, -3, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,  0, -1, -1, -1, -1,  0,  0, -1, -1, -4, -1, -2, -1,  
         0, -2, -2, -2, -2, -1, -1, -2,  4,  3, -2,  2,  2, -2, -1, -1, -2, -2, -1,  0, -1,  4, -6, -1, -2, -2,  
        -6, -5, -8, -7, -7,  0, -7, -3, -5, -4, -3, -2, -4, -4, -4, -6, -5,  2, -2, -5, -4, -6, 17, -4,  0, -6,  
         0, -1, -3, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,  0, -1, -1, -1, -1,  0,  0, -1, -1, -4, -1, -2, -1,  
        -3, -3,  0, -4, -4,  7, -5,  0, -1, -1, -4, -1, -2, -2, -2, -5, -4, -4, -3, -3, -2, -2,  0, -2, 10, -4,  
         0,  2, -5,  3,  3, -5,  0,  2, -2, -3,  0, -3, -2,  1, -1,  0,  3,  0,  0, -1, -1, -2, -6, -1, -4,  3,  
         
    ]).unwrap();
}

pub fn pam250(a: &u8, b: &u8) -> i32 {
    PAM250[((*a as usize) - 65, (*b as usize) - 65)]
}


lazy_static! {
    static ref UNIT: ndarray::Array2<i32> = ndarray::Array::from_shape_vec((26, 26), vec![
         1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1, -1,  
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  1,  
    ]).unwrap();
}

pub fn unit(a: &u8, b: &u8) -> i32 {
    UNIT[((*a as usize) - 65, (*b as usize) - 65)]
}