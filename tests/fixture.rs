use std::fs;
use std::path::PathBuf;

use opengraph_transform::{main, TransformVisitor};
use swc_core::common::{chain, Mark};
use swc_core::{
    ecma::parser::{EsConfig, Syntax},
    ecma::transforms::base::resolver,
    ecma::transforms::testing::test_fixture,
    ecma::visit::as_folder,
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_ascii_whitespace())
}
#[fixture("tests/fixture/**/code.jsx")]
fn tranform_works(input: PathBuf) {
    let output = input.parent().unwrap().join("output.jsx");

    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(TransformVisitor::default())
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
#[fixture("tests/fixture/typescript/**/code.tsx")]
fn typescript(input: PathBuf) {
    let output = input.parent().unwrap().join("output.tsx");
    let in_contents = fs::read_to_string(input).unwrap();
    let mut out_contents = fs::read_to_string(output).unwrap();
    let mut result = main(in_contents, "code.tsx".into());
    remove_whitespace(&mut result);
    remove_whitespace(&mut out_contents);
    assert_eq!(result, out_contents);
}
