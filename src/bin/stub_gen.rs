fn main() -> pyo3_stub_gen::Result<()> {
    let stub = slai::stub_info()?;
    stub.generate()?;
    Ok(())
}
