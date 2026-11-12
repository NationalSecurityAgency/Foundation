#[cfg(test)]
mod tests {

    use crate::{get_array, get_vec, output_array, output_vec};
    use std::ptr::addr_of;

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::needless_range_loop)]
    fn linear_fill(a: &mut [u8]) {
        for i in 0..a.len() {
            a[i] = i as u8;
        }
    }

    #[test]
    fn get_array_copies_values_correctly() {
        let mut src: [u8; 500] = [0; 500];
        linear_fill(&mut src);

        // SAFETY: src is a live, aligned array containing 500 initialized bytes.
        let result = unsafe { get_array(&src as *const u8) };
        assert_ne!(addr_of!(src), addr_of!(result));
        assert_eq!(src, result);
    }

    #[test]
    fn get_vec_copies_values_correctly() {
        let mut src: [u8; 500] = [0; 500];
        linear_fill(&mut src);

        // SAFETY: src is a live, aligned array containing 500 initialized bytes.
        let result = unsafe { get_vec(500, &src as *const u8) };
        assert_eq!(src.as_slice(), result);
    }

    #[test]
    fn output_array_copies_values_correctly() {
        let mut src: [u8; 500] = [0; 500];
        let mut dst: [u8; 500] = [0; 500];
        linear_fill(&mut src);

        // SAFETY: dst is a live, aligned array with space for all 500 bytes.
        unsafe { output_array(&src, &mut dst as *mut u8) };
        assert_eq!(src, dst);
    }

    #[test]
    fn output_vec_copies_values_correctly() {
        let mut src: Vec<u8> = vec![0; 500];
        let mut dst: [u8; 500] = [0; 500];
        linear_fill(src.as_mut_slice());

        // SAFETY: dst is a live, aligned array with space for src.len() bytes.
        unsafe { output_vec(&src, &mut dst as *mut u8) };
        assert_eq!(src.as_slice(), dst);
    }
}
