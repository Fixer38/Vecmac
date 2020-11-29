#[macro_export]
macro_rules! vecmac {
    // First pattern is for as many elements as we want to init the vec
    // Second pattern to avoid the removal of coma in case there is trailing
    ($($element:expr),* $(,)*) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        // Repeat the pattern as many times as element is in it
        // Looks for the number of time the pattern element is in it
        // If multiple variables, requestion is that they must repeat the same amount of time
        // + = 0 or more, * = 1 or more
        $(vs.push($element);)*
        vs
    }};

    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $element;
        for _ in 0..$count {
            vs.push(x.clone());
        }
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


#[test]
fn clone_2() {
    let x: Vec<u32> = vecmac![35; 3];
    assert_eq!(x[0], 35);
    assert_eq!(x[1], 35);
}


#[test]
fn clone_2_non_literal() {
    let mut y = Some(35);
    let x: Vec<u32> = vecmac![y.take().unwrap(); 3];
    assert_eq!(x[0], 35);
    assert_eq!(x[1], 35);
}


