use crystometer::CrystalWrapper;

fn main() -> Result<(), &'static str> {
    let mut crystal = CrystalWrapper::new();
    crystal.rotate(45.0)?;
    Ok(())
}
