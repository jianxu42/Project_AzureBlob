type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() -> TestResult {
    Ok(())
}
