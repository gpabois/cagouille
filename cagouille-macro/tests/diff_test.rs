use cagouille::{df::traits::Differentiable, prelude::*};

#[derive(Differentiable)]
pub struct Bar{attr: String}

#[test]
fn test_diff_derive() {
    let b1 = Bar{attr: "test_1".into()};
    let b2 = Bar{attr: "test_2".into()};

    let df = Bar::df(&b1, &b2);

    assert!(df.attr.is_some());
    assert!(df.attr.unwrap() == "test_2".to_string());
} 