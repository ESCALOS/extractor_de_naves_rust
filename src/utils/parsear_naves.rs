use crate::{model::Nave, utils::parsear_linea_nave};
use super::obtener_mes_anterior;

pub fn parsear_naves(texto: &str) -> Result<Vec<Nave>, Box<dyn std::error::Error>> {
    let mut naves = Vec::new();
    
    // Buscar el primer mes explícito en todo el documento
    let mut primer_mes_explicito = String::new();
    
    for linea in texto.lines() {
        let linea = linea.trim();
        
        // Buscar líneas de mes explícito (solo nombres de mes en líneas cortas)
        if linea.len() < 30 {
            if linea.contains("DICIEMBRE") {
                primer_mes_explicito = "DICIEMBRE".to_string();
                break;
            } else if linea.contains("ENERO") {
                primer_mes_explicito = "ENERO".to_string();
                break;
            } else if linea.contains("FEBRERO") {
                primer_mes_explicito = "FEBRERO".to_string();
                break;
            } else if linea.contains("MARZO") {
                primer_mes_explicito = "MARZO".to_string();
                break;
            } else if linea.contains("ABRIL") {
                primer_mes_explicito = "ABRIL".to_string();
                break;
            } else if linea.contains("MAYO") {
                primer_mes_explicito = "MAYO".to_string();
                break;
            } else if linea.contains("JUNIO") {
                primer_mes_explicito = "JUNIO".to_string();
                break;
            } else if linea.contains("JULIO") {
                primer_mes_explicito = "JULIO".to_string();
                break;
            } else if linea.contains("AGOSTO") {
                primer_mes_explicito = "AGOSTO".to_string();
                break;
            } else if linea.contains("SEPTIEMBRE") {
                primer_mes_explicito = "SEPTIEMBRE".to_string();
                break;
            } else if linea.contains("OCTUBRE") {
                primer_mes_explicito = "OCTUBRE".to_string();
                break;
            } else if linea.contains("NOVIEMBRE") {
                primer_mes_explicito = "NOVIEMBRE".to_string();
                break;
            }
        }
    }
    
    // Determinar el mes inicial: mes anterior al primer mes explícito, o octubre si no hay ninguno
    let mut mes_actual = if !primer_mes_explicito.is_empty() {
        obtener_mes_anterior(&primer_mes_explicito)
    } else {
        "OCTUBRE".to_string() // Si no hay mes explícito, usar el mes actual
    };
    
    // Procesar las naves
    let mut en_seccion_arrivals = false;

    for linea in texto.lines() {
        let linea = linea.trim();

        // Detectar secciones de arrivals
        if linea.contains("VESSEL ARRIVALS") {
            en_seccion_arrivals = true;
            continue;
        }

        // Detectar líneas de mes explícito y cambiar el mes actual
        if linea.len() < 30 {
            if linea.contains("DICIEMBRE") {
                mes_actual = "DICIEMBRE".to_string();
                continue;
            } else if linea.contains("ENERO") {
                mes_actual = "ENERO".to_string();
                continue;
            } else if linea.contains("FEBRERO") {
                mes_actual = "FEBRERO".to_string();
                continue;
            } else if linea.contains("MARZO") {
                mes_actual = "MARZO".to_string();
                continue;
            } else if linea.contains("ABRIL") {
                mes_actual = "ABRIL".to_string();
                continue;
            } else if linea.contains("MAYO") {
                mes_actual = "MAYO".to_string();
                continue;
            } else if linea.contains("JUNIO") {
                mes_actual = "JUNIO".to_string();
                continue;
            } else if linea.contains("JULIO") {
                mes_actual = "JULIO".to_string();
                continue;
            } else if linea.contains("AGOSTO") {
                mes_actual = "AGOSTO".to_string();
                continue;
            } else if linea.contains("SEPTIEMBRE") {
                mes_actual = "SEPTIEMBRE".to_string();
                continue;
            } else if linea.contains("OCTUBRE") {
                mes_actual = "OCTUBRE".to_string();
                continue;
            } else if linea.contains("NOVIEMBRE") {
                mes_actual = "NOVIEMBRE".to_string();
                continue;
            }
        }

        // Saltar líneas de encabezado
        if linea.contains("ETA SHIPS ETB BERTH") || linea.is_empty() || 
           linea.contains("LISTADO DE") || linea.contains("Classification") ||
           linea.contains("TYPE RORO") || linea.contains("Row Labels") {
            continue;
        }

        // Parsear líneas de datos de naves (buscar líneas que empiecen con 6 dígitos)
        if en_seccion_arrivals && linea.len() > 50 {
            // Verificar si la línea empieza con ETA (6 dígitos: ddHHmm)
            let primera_palabra = linea.split_whitespace().next().unwrap_or("");
            if primera_palabra.len() == 6 && primera_palabra.chars().all(|c| c.is_numeric()) {
                if let Some(nave) = parsear_linea_nave(linea, &mes_actual) {
                    naves.push(nave);
                }
            }
        }
    }

    Ok(naves)
}