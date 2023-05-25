fn main() {
    let little_endian = cfg!(target_endian = "little");
    if !little_endian {
        assert!(cfg!(target_endian = "big"));
    }
    dbg!(little_endian);

    let c = u32::from_ne_bytes([0, 1, 2, 3]);
    println!("{} 0x{:x}", c, c);

    let ne = c.to_ne_bytes();
    let le = c.to_le_bytes();
    let be = c.to_be_bytes();
    dbg!(ne, le, be);

    let slice: &[u32] = &[c];
    let casted: &[u8] = bytemuck::cast_slice(slice);
    dbg!(casted);
}
