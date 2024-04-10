/// Reads wide chars until `0` is encountered.
pub fn read_until_nul<const N: usize>(ptr: *const [u16; N]) -> Vec<u16> {
    let mut vec = Vec::with_capacity(N);
    let ptr = ptr.cast::<u16>();
    for i in 0..N {
        let value = unsafe { ptr.add(i).read_volatile() };
        if value == 0 {
            break;
        } else {
            vec.push(value);
        }
    }
    vec
}

/// Returns the subslice until the first `0`.
pub fn until_nul(slice: &[u16]) -> &[u16] {
    let end = slice.iter().position(|el| *el == 0).unwrap_or(slice.len());
    &slice[..end]
}
