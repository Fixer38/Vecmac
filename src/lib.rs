#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new();
    }
}



#[test]
fn empty_vec() {
    let x: Vec<u32> = vecmac![];
    assert!(x.is_empty());
}
