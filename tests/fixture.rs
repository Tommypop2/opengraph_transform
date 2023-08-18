use std::path::PathBuf;

use opengraph_transform::TransformVisitor;
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

#[fixture("tests/fixture/**/code.jsx")]
fn jsx_dom_expressions_fixture_babel(input: PathBuf) {
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

#[test]
fn yes() {
    opengraph_transform::main("asd".into(), "file.tsx".into());
    assert_eq!(true, false);
}