#[cfg(kani)]
mod proofs {

    use crate::{get_array, get_vec, output_array, output_vec};

    fn get_array_valid_pointer_contract<T: Copy + Default + PartialEq, const N: usize>() {
        let src: [T; N] = [Default::default(); N];
        // SAFETY: src is live, aligned, and initialized for N values.
        let result: [T; N] = unsafe { get_array(&src as *const T) };
        assert_eq!(&src, &result);
    }

    #[kani::proof]
    fn get_array_contract() {
        get_array_valid_pointer_contract::<u8, 0>();
        get_array_valid_pointer_contract::<u8, 1>();
        get_array_valid_pointer_contract::<u8, 4>();
        get_array_valid_pointer_contract::<u8, 100>();
        get_array_valid_pointer_contract::<i8, 4>();
        get_array_valid_pointer_contract::<u32, 8>();
        get_array_valid_pointer_contract::<u128, 8>();
        get_array_valid_pointer_contract::<char, 5>();
    }

    fn get_vec_valid_pointer_contract<T: Copy + Default + PartialEq, const N: usize>() {
        let src: [T; N] = [Default::default(); N];
        // SAFETY: src is live, aligned, and initialized for N values.
        let result = unsafe { get_vec(N, &src as *const T) };
        assert_eq!(&src[..], result.as_slice());
    }

    #[kani::proof]
    fn get_vec_contract() {
        get_vec_valid_pointer_contract::<u8, 0>();
        get_vec_valid_pointer_contract::<u8, 1>();
        get_vec_valid_pointer_contract::<u8, 4>();
        get_vec_valid_pointer_contract::<u8, 100>();
        get_vec_valid_pointer_contract::<i8, 4>();
        get_vec_valid_pointer_contract::<u32, 8>();
        get_vec_valid_pointer_contract::<u128, 8>();
        get_vec_valid_pointer_contract::<char, 5>();
    }

    fn output_array_valid_pointer_contract<T: Copy + Default + PartialEq, const N: usize>() {
        let src: [T; N] = [Default::default(); N];
        let mut dst: [T; N] = [Default::default(); N];
        // SAFETY: dst is live, aligned, and writable for N values.
        unsafe { output_array(&src, &mut dst as *mut T) };
        assert_eq!(&src, &dst);
    }

    #[kani::proof]
    fn output_array_contract() {
        output_array_valid_pointer_contract::<u8, 0>();
        output_array_valid_pointer_contract::<u8, 1>();
        output_array_valid_pointer_contract::<u8, 4>();
        output_array_valid_pointer_contract::<u8, 100>();
        output_array_valid_pointer_contract::<i8, 4>();
        output_array_valid_pointer_contract::<u32, 8>();
        output_array_valid_pointer_contract::<u128, 8>();
        output_array_valid_pointer_contract::<char, 5>();
    }

    fn output_vec_valid_pointer_contract<T: Copy + Default + PartialEq, const N: usize>() {
        let src: Vec<T> = vec![Default::default(); N];
        let mut dst: [T; N] = [Default::default(); N];
        // SAFETY: dst is live, aligned, and writable for N values.
        unsafe { output_vec(&src, &mut dst as *mut T) };
        assert_eq!(&src, &dst);
    }

    #[kani::proof]
    fn output_vec_contract() {
        output_vec_valid_pointer_contract::<u8, 0>();
        output_vec_valid_pointer_contract::<u8, 1>();
        output_vec_valid_pointer_contract::<u8, 4>();
        output_vec_valid_pointer_contract::<u8, 100>();
        output_vec_valid_pointer_contract::<i8, 4>();
        output_vec_valid_pointer_contract::<u32, 8>();
        output_vec_valid_pointer_contract::<u128, 8>();
        output_vec_valid_pointer_contract::<char, 5>();
    }
}
