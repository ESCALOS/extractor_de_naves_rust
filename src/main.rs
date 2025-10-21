mod utils;
mod model;

use std::fs;

use reqwest;
use crate::utils::{exportar_a_excel, parsear_naves, procesar_naves_para_excel, leer_agentes_excel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Leer la URL del archivo .env
    let timestamp = chrono::Local::now().timestamp();
    let url = format!("https://cms-cd.apmterminals.com/callao/-/media/mainsite/americas/Callao/daily-updates/listado-de-naves.pdf?rev={}", timestamp);

    println!("Iniciando proceso de extracción de naves...");

    // Leer información de agentes
    println!("Leyendo información de agentes...");
    let agentes = leer_agentes_excel("AGENTES.xlsx")?;

    // Descargar el PDF
    let pdf_bytes = descargar_pdf(&url).await?;

    // Guardar el PDF para referencia
    fs::write("listado_de_naves.pdf", &pdf_bytes)?;
    println!("PDF guardado como 'listado_de_naves.pdf'");

    // Extraer texto del PDF
    let texto = utils::extraer_texto_pdf(&pdf_bytes)?;

    // Parsear los datos de las naves
    let naves = parsear_naves(&texto)?;

    // Procesar las naves para Excel con filtros
    let naves_excel = procesar_naves_para_excel(&naves, &agentes)?;

    // Exportar a Excel
    exportar_a_excel(&naves_excel)?;

    Ok(())
}

async fn descargar_pdf(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}