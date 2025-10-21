use calamine::{Reader, Xlsx, open_workbook};
use calamine::DataType;
use crate::model::Agente;
use std::path::Path;

pub fn leer_agentes_excel(ruta_archivo: &str) -> Result<Vec<Agente>, Box<dyn std::error::Error>> {
    // Verificar si el archivo existe
    if !Path::new(ruta_archivo).exists() {
        println!("Archivo '{}' no encontrado. Continuando sin información de agentes.", ruta_archivo);
        return Ok(Vec::new());
    }

    let mut workbook: Xlsx<_> = match open_workbook(ruta_archivo) {
        Ok(wb) => wb,
        Err(e) => {
            println!("Error al abrir el archivo '{}': {}. Continuando sin información de agentes.", ruta_archivo, e);
            return Ok(Vec::new());
        }
    };
    
    let mut agentes = Vec::new();
    
    // Obtener la primera hoja
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let mut filas = range.rows();
        
        // Saltar la fila de encabezados
        filas.next();
        
        for fila in filas {
            if fila.len() >= 5 {
                let operador = fila[0].get_string().unwrap_or("").trim().to_string();
                let cargo = fila[1].get_string().unwrap_or("").trim().to_string();
                let nombre_apellido = fila[2].get_string().unwrap_or("").trim().to_string();
                let telefono = fila[3].to_string().trim().to_string();
                let correo = fila[4].get_string().unwrap_or("").trim().to_string();
                
                // Solo agregar si el operador no está vacío
                if !operador.is_empty() {
                    let agente = Agente {
                        operador,
                        cargo,
                        nombre_apellido,
                        telefono,
                        correo,
                    };
                    agentes.push(agente);
                }
            }
        }
    }
    
    println!("Leídos {} agentes del archivo Excel", agentes.len());
    Ok(agentes)
}

pub fn buscar_agente_por_operador<'a>(agentes: &'a [Agente], operador_cargo: &str) -> Option<&'a Agente> {
    // Convertir a mayúsculas para comparación insensible a mayúsculas/minúsculas
    let operador_cargo_upper = operador_cargo.to_uppercase();
    
    agentes.iter().find(|agente| {
        operador_cargo_upper.contains(&agente.operador.to_uppercase())
    })
}