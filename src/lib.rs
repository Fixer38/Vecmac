#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new()
    };

    ($($element:expr), +) => {{
        let mut vs = Vec::new();
        // Repeat the pattern as many times as element is in it
        // Looks for the number of time the pattern element is in it
        $(vs.push($element);)+
        vs
    }};
}



#[test]
fn empty_vec() {
    let x: Vec<u32> = vecmac![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = vecmac![35];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 35);
}

#[test]
fn double() {
    let x: Vec<u32> = vecmac![35, 41];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 35);
    assert_eq!(x[1], 41);
}
