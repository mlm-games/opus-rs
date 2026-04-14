//! Fixed-point arithmetic utilities for CELT optimization
//! 
//! This module provides fixed-point operations compatible with C opus FIXED_POINT mode.
//! Uses Q15 format (1.15) for 16-bit values and Q31 for 32-bit values.

/// Q15 format: 1 sign bit, 15 fractional bits
pub const Q15_SHIFT: i32 = 15;
pub const Q15_ONE: i32 = 32767;

/// Q31 format: 1 sign bit, 31 fractional bits  
pub const Q31_SHIFT: i32 = 31;
pub const Q31_ONE: i32 = 2147483647;

/// NORM_SHIFT used in C opus
pub const NORM_SHIFT: i32 = 24;

/// Saturate a 32-bit value to 16-bit
#[inline(always)]
pub fn sat16(x: i32) -> i16 {
    if x > 32767 {
        32767
    } else if x < -32768 {
        -32768
    } else {
        x as i16
    }
}

/// Saturate a 64-bit value to 32-bit
#[inline(always)]
pub fn sat32(x: i64) -> i32 {
    if x > i32::MAX as i64 {
        i32::MAX
    } else if x < i32::MIN as i64 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Multiply two Q15 values, result in Q15
/// Equivalent to MULT16_16_Q15
#[inline(always)]
pub fn mul_q15(a: i16, b: i16) -> i16 {
    // (a * b + rounding) >> 15
    let prod = (a as i32 * b as i32 + 16384) >> 15;
    prod as i16
}

/// Multiply two Q15 values, result in Q31 (no rounding)
/// Equivalent to MULT16_16
#[inline(always)]
pub fn mul16_16(a: i16, b: i16) -> i32 {
    a as i32 * b as i32
}

/// Multiply Q15 by Q15, result in Q14 (for accumulation)
/// Equivalent to MULT16_16_Q14
#[inline(always)]
pub fn mul_q14(a: i16, b: i16) -> i16 {
    ((a as i32 * b as i32 + 8192) >> 14) as i16
}

/// 16x32 multiply, result in Q16
/// Equivalent to MULT16_32_Q16
#[inline(always)]
pub fn mul16_32_q16(a: i16, b: i32) -> i32 {
    // (a * b) >> 16
    ((a as i64 * b as i64) >> 16) as i32
}

/// Shift left with saturation
/// Equivalent to SHL32
#[inline(always)]
pub fn shl32(a: i32, shift: i32) -> i32 {
    if shift <= 0 {
        a >> (-shift)
    } else {
        let result = (a as i64) << shift;
        sat32(result)
    }
}

/// Shift right (arithmetic)
/// Equivalent to SHR32
#[inline(always)]
pub fn shr32(a: i32, shift: i32) -> i32 {
    if shift <= 0 {
        a << (-shift)
    } else {
        a >> shift
    }
}

/// Shift right with rounding
/// Equivalent to PSHR32
#[inline(always)]
pub fn pshr32(a: i32, shift: i32) -> i32 {
    if shift <= 0 {
        a << (-shift)
    } else {
        (a + (1 << (shift - 1))) >> shift
    }
}

/// Absolute value for i16
#[inline(always)]
pub fn abs16(x: i16) -> i16 {
    if x < 0 { -x } else { x }
}

/// Absolute value for i32
#[inline(always)]
pub fn abs32(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

/// Convert f32 to Q15 i16
#[inline(always)]
pub fn float_to_q15(x: f32) -> i16 {
    let scaled = x * 32767.0;
    sat16(scaled as i32)
}

/// Convert Q15 i16 to f32
#[inline(always)]
pub fn q15_to_float(x: i16) -> f32 {
    x as f32 / 32767.0
}

/// Fixed-point reciprocal approximation
/// Returns Q15 reciprocal of a Q15 number
#[inline(always)]
pub fn recip_q15(x: i16) -> i16 {
    if x <= 0 {
        return Q15_ONE as i16;
    }
    
    // Use Newton-Raphson iteration for 1/x
    // Initial approximation using lookup table or simple method
    let x_f = x as f32 / 32767.0;
    let recip_f = 1.0 / x_f;
    float_to_q15(recip_f)
}

/// Multiply-accumulate in Q15: acc += a * b
#[inline(always)]
pub fn mac_q15(acc: i32, a: i16, b: i16) -> i32 {
    acc + mul16_16(a, b)
}

/// Square a Q15 value, result in Q31
#[inline(always)]
pub fn sqr_q15(a: i16) -> i32 {
    let a_i32 = a as i32;
    a_i32 * a_i32
}

/// Compute sum of squares for a slice of Q15 values
/// Returns Q30 result (to avoid overflow)
pub fn sum_sqr_q15(x: &[i16]) -> i32 {
    let mut sum: i64 = 0;
    for &v in x {
        let v_i32 = v as i32;
        sum += (v_i32 * v_i32) as i64;
    }
    // Scale down to Q30 (or Q31)
    (sum >> 15) as i32
}

/// Dot product of two Q15 slices
/// Returns Q30 result
pub fn dot_product_q15(a: &[i16], b: &[i16]) -> i32 {
    assert_eq!(a.len(), b.len());
    let mut sum: i64 = 0;
    for i in 0..a.len() {
        sum += (a[i] as i32 * b[i] as i32) as i64;
    }
    (sum >> 15) as i32
}

/// Find maximum absolute value in a slice
pub fn maxabs16(x: &[i16]) -> i16 {
    let mut max_val: i16 = 0;
    let mut min_val: i16 = 0;
    
    for &v in x {
        if v > max_val {
            max_val = v;
        }
        if v < min_val {
            min_val = v;
        }
    }
    
    if max_val > -min_val {
        max_val
    } else {
        -min_val
    }
}

/// Normalize a vector to Q15 with proper scaling
/// Returns the scale factor to convert back: original = q15_to_float(q15) * max_val
pub fn normalize_to_q15(x: &[f32], out: &mut [i16], max_val_out: &mut f32) {
    assert_eq!(x.len(), out.len());
    
    // Find max
    let mut max_val = 0.0f32;
    for &v in x {
        let abs_v = v.abs();
        if abs_v > max_val {
            max_val = abs_v;
        }
    }
    
    *max_val_out = max_val;
    
    if max_val < 1e-15 {
        // All zeros
        out.fill(0);
        return;
    }
    
    // Scale to Q15 range
    let scale = 32767.0 / max_val;
    
    for i in 0..x.len() {
        let scaled = x[i] * scale;
        out[i] = sat16(scaled as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sat16() {
        assert_eq!(sat16(1000), 1000);
        assert_eq!(sat16(40000), 32767);
        assert_eq!(sat16(-40000), -32768);
    }

    #[test]
    fn test_mul_q15() {
        // 0.5 * 0.5 = 0.25
        let a = float_to_q15(0.5);
        let b = float_to_q15(0.5);
        let c = mul_q15(a, b);
        let c_f = q15_to_float(c);
        assert!((c_f - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_normalize() {
        let x = [0.0, 0.5, 1.0, -0.5, -1.0];
        let mut out = [0i16; 5];
        let mut max_val = 0.0;
        
        normalize_to_q15(&x, &mut out, &mut max_val);
        
        // Max should be close to Q15_ONE
        let max_out = maxabs16(&out);
        assert!(max_out > 32000);
        
        // Verify round-trip: q15_to_float(out[i]) * max_val should approx equal x[i]
        // (allowing for quantization error)
        for i in 0..x.len() {
            let back = q15_to_float(out[i]) * max_val;
            assert!((back - x[i]).abs() < 0.02, "Mismatch at {}: {} vs {}", i, back, x[i]);
        }
    }
}
