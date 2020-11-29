#[macro_export]
macro_rules! vecmac {
    () => {
        Vec::new()
    };

    // First pattern is for as many elements as we want to init the vec
    // Second pattern to avoid the removal of coma in case there is trailing
    ($($element:expr), + $(,)*) => {{
        let mut vs = Vec::new();
        // Repeat the pattern as many times as element is in it
        // Looks for the number of time the pattern element is in it
        // If multiple variables, requestion is that they must repeat the same amount of time
        // + = 0 or more, * = 1 or more
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

// Allow trailing coma at the end of a long list of elements
#[test]
fn trailing() {
    let x: Vec<u32> = vecmac![35, 41,];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 35);
    assert_eq!(x[1], 41);
}
