//! Tests that errors are properly reported when invalid data is provided

use climake::{Argument, CLIMake, DataType};

/// Makes sure that passing no calltypes to an argument, e.g. `vec![], vec![]`
/// will return a [CLIError::NoCalls]
#[test]
#[should_panic]
fn ensure_nocalls_error() {
    Argument::new(
        vec![],
        vec![],
        Some("This should return a CLIError::NoCalls once added to cli"),
        DataType::None,
    )
    .unwrap();
}

/// Makes sure that giving duplicated arguments with same call types will
/// result in a [CLIError::ArgExists]
#[test]
#[should_panic]
fn ensure_dupe_error() {
    let org_arg = Argument::new(
        vec!['d'],
        vec![String::from("dupecall")],
        None,
        DataType::None,
    )
    .unwrap();

    CLIMake::new(vec![org_arg.clone(), org_arg], None, None).unwrap(); // will fail with cloned org_arg
}