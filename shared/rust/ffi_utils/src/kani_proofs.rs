#[cfg(kani)]
mod proofs {

    use crate::{get_array, get_vec, output_array, output_vec};

    fn get_array_safety_check<T: Copy + Default + PartialEq, const N: usize>() {
        let src: [T; N] = [Default::default(); N];
        let result: [T; N] = get_array(&src as *const T);
        assert_eq!(&src, &result);
    }

    #[kani::proof]
    fn get_array_safe() {
        get_array_safety_check::<u8, 0>();
        get_array_safety_check::<u8, 1>();
        get_array_safety_check::<u8, 4>();
        get_array_safety_check::<u8, 100>();
        get_array_safety_check::<i8, 4>();
        get_array_safety_check::<u32, 8>();
        get_array_safety_check::<u128, 8>();
        get_array_safety_check::<char, 5>();
    }

    fn get_vec_safety_check<T: Copy + Default + PartialEq, const N: usize>() {
        let src: [T; N] = [Default::default(); N];
        let result = get_vec(N, &src as *const T);
        assert_eq!(&src[..], result.as_slice());
    }

    #[kani::proof]
    fn get_vec_safe() {
        get_vec_safety_check::<u8, 0>();
        get_vec_safety_check::<u8, 1>();
        get_vec_safety_check::<u8, 4>();
        get_vec_safety_check::<u8, 100>();
        get_vec_safety_check::<i8, 4>();
        get_vec_safety_check::<u32, 8>();
        get_vec_safety_check::<u128, 8>();
        get_vec_safety_check::<char, 5>();
    }

    fn output_array_safety_check<T: Copy + Default + PartialEq, const N: usize>() {
        let src: [T; N] = [Default::default(); N];
        let mut dst: [T; N] = [Default::default(); N];
        output_array(&src, &mut dst as *mut T);
        assert_eq!(&src, &dst);
    }

    #[kani::proof]
    fn output_array_safe() {
        output_array_safety_check::<u8, 0>();
        output_array_safety_check::<u8, 1>();
        output_array_safety_check::<u8, 4>();
        output_array_safety_check::<u8, 100>();
        output_array_safety_check::<i8, 4>();
        output_array_safety_check::<u32, 8>();
        output_array_safety_check::<u128, 8>();
        output_array_safety_check::<char, 5>();
    }

    fn output_vec_safety_check<T: Copy + Default + PartialEq, const N: usize>() {
        let src: Vec<T> = vec![Default::default(); N];
        let mut dst: [T; N] = [Default::default(); N];
        output_vec(&src, &mut dst as *mut T);
        assert_eq!(&src, &dst);
    }

    #[kani::proof]
    fn output_vec_safe() {
        output_vec_safety_check::<u8, 0>();
        output_vec_safety_check::<u8, 1>();
        output_vec_safety_check::<u8, 4>();
        output_vec_safety_check::<u8, 100>();
        output_vec_safety_check::<i8, 4>();
        output_vec_safety_check::<u32, 8>();
        output_vec_safety_check::<u128, 8>();
        output_vec_safety_check::<char, 5>();
    }
}
