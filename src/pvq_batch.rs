// Batch U value computation for icwrs
// Pre-computes multiple U values at once to amortize overhead

use crate::pvq::{CELT_PVQ_U_DATA, CELT_PVQ_U_ROW, ncwrs};

/// Batch compute U values for icwrs
/// Fills u_values[m] = U(m, k) for m in 2..=n
#[inline(always)]
pub fn compute_u_batch(n: u32, k: u32, u_values: &mut [u32]) {
    // For now, just use individual lookups
    // TODO: Optimize with actual batch computation
    for m in 2..=n {
        let r = m.min(k) as usize;
        let c = m.max(k) as usize;
        
        if r < 15 && CELT_PVQ_U_ROW[r] as usize + c < CELT_PVQ_U_DATA.len() {
            let idx = CELT_PVQ_U_ROW[r] as usize + c;
            u_values[m as usize] = CELT_PVQ_U_DATA[idx];
        } else {
            u_values[m as usize] = ncwrs(m, k);
        }
    }
}
