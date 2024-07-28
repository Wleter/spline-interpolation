///  The function evaluates the (k+1) non-zero b-splines of
///  degree k at t[l] <= x < t[l+1] using the stable recurrence
///  relation of de boor and cox.
///  that weighting of 0 is used when knots with multiplicity are present.
///  Also, notice that l+k <= n and 1 <= l+1-k
///      or else the routine will be accessing memory outside t
///      Thus it is imperative that that k <= l <= n-k but this
///      is not checked.
///  The original subroutine in FITPACK by Paul Dierckx is named fpbspl
pub fn fpbspl(x: f64, t: &[f64], k: usize, l: usize) -> Vec<f64> {
    let mut h: [f64; 20] = [0.0; 20];
    let mut hh: [f64; 19] = [0.0; 19];
    h[0] = 1.0;
    for j in 1..(k + 1) {
        for i in 1..(j + 1) {
            hh[i - 1] = h[i - 1];
        }
        h[0] = 0.0;
        for i in 1..(j + 1) {
            let li: usize = l + i;
            let lj: usize = li - j;
            if t[li - 1] == t[lj - 1] {
                h[i] = 0.0;
            } else {
                let f: f64 = hh[i - 1] / (t[li - 1] - t[lj - 1]);
                h[i - 1] = h[i - 1] + f * (t[li - 1] - x);
                h[i] = f * (x - t[lj - 1]);
            }
        }
    }
    return h.to_vec();
}
