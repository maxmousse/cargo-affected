fn main() -> anyhow::Result<()> {
    let metadata = core::metadata::get_metadata()?;

    println!("Metadata: {:#?}", metadata);

    Ok(())
}
