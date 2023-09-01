use all_features::test;

#[test]
fn bind() {
    let test = <test as polygen::__private::ExportedPolyFn>::FUNCTION;
    panic!("{}::{}", test.module, test.ident);
}
