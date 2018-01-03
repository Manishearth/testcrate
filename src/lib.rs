extern "C" {
    fn foo();
}

#[test]
fn test() {
    unsafe {foo()}
}
