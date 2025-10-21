pub fn extraer_texto_pdf(pdf_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let texto = pdf_extract::extract_text_from_mem(pdf_bytes)?;
    Ok(texto)
}