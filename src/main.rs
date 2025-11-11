mod utils;
mod model;

use std::fs;
use std::io::{self, Write};

use reqwest;
use clap::Parser;
use crate::utils::{exportar_a_excel, parsear_naves, procesar_naves_para_excel, leer_agentes_excel};

#[derive(Parser)]
#[command(name = "Extractor de Naves")]
#[command(about = "Extrae y procesa información de naves desde PDF", long_about = None)]
struct Args {
    /// Número mínimo de días en bahía para filtrar las naves
    #[arg(short = 'd', long = "dias")]
    min_dias_bahia: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Si no se pasó el argumento, preguntar al usuario
    let min_dias_bahia = match args.min_dias_bahia {
        Some(dias) => dias,
        None => {
            print!("Ingrese el número mínimo de días en bahía [por defecto 5]: ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let dias = input.trim();
            if dias.is_empty() {
                5 // Valor por defecto
            } else {
                dias.parse().unwrap_or_else(|_| {
                    println!("Valor inválido, usando 5 días por defecto");
                    5
                })
            }
        }
    };

    // Leer la URL del archivo .env
    let timestamp = chrono::Local::now().timestamp();
    let url = format!("https://cms-cd.apmterminals.com/callao/-/media/mainsite/americas/Callao/daily-updates/listado-de-naves.pdf?rev={}", timestamp);

    println!("Iniciando proceso de extracción de naves...");
    println!("Filtrando naves con al menos {} días en bahía", min_dias_bahia);

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
    let naves_excel = procesar_naves_para_excel(&naves, &agentes, min_dias_bahia)?;

    // Exportar a Excel
    exportar_a_excel(&naves_excel)?;

    println!("\n¡Proceso completado! Presione Enter para salir...");
    let mut pausa = String::new();
    io::stdin().read_line(&mut pausa)?;

    Ok(())
}

async fn descargar_pdf(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}