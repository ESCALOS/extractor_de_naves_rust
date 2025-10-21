use crate::model::Nave;

pub fn parsear_linea_nave(linea: &str, mes: &str) -> Option<Nave> {
    // Dividir la línea en campos usando espacios
    let campos: Vec<&str> = linea.split_whitespace().collect();

    if campos.len() < 10 {
        return None;
    }

    // ETA está en la primera posición
    let eta = campos[0].to_string();

    // Buscar el ETB (segundo número de 6 dígitos)
    let mut etb_index = 0;
    for (i, campo) in campos.iter().enumerate().skip(1) {
        if campo.len() == 6 && campo.chars().all(|c| c.is_numeric()) {
            etb_index = i;
            break;
        }
    }

    if etb_index == 0 {
        return None;
    }

    // El nombre del barco está entre ETA y ETB
    let ship_name = campos[1..etb_index].join(" ");
    let etb = campos[etb_index].to_string();

    // Buscar el tipo de nave (después del berth y LOA/BEAM)
    // Solo buscamos RORO, GRANELERO, GENERAL o MULTIPROPOSITO de manera exacta
    let mut tipo = String::new();
    let mut tipo_index = 0;
    
    for (i, campo) in campos.iter().enumerate().skip(etb_index + 3) {
        if *campo == "RORO" || *campo == "GRANELERO" || *campo == "GENERAL" {
            tipo = campo.to_string();
            tipo_index = i;
            break;
        }
    }

    // Si no encontramos uno de los tipos que nos interesan, omitir la línea
    if tipo.is_empty() {
        return None;
    }

    // Capturar todo lo que está después del tipo (operador, cargo y cantidad juntos)
    let resto_campos = if tipo_index + 1 < campos.len() {
        campos[(tipo_index + 1)..].join(" ")
    } else {
        String::new()
    };

    // Función auxiliar para cortar quantity hasta encontrar TM, UND, UNID o MT
    fn cortar_quantity(quantity_text: &str) -> String {
        let palabras: Vec<&str> = quantity_text.split_whitespace().collect();
        let mut resultado = Vec::new();
        
        for palabra in palabras {
            resultado.push(palabra);
            // Si la palabra termina con alguna de estas unidades, parar aquí
            if palabra.ends_with("TM") || palabra.ends_with("UND") || 
               palabra.ends_with("UNID") || palabra.ends_with("MT") {
                break;
            }
        }
        
        resultado.join(" ")
    }

    // Separar por D/, E/ o D./
    let (operador_cargo, quantity) = if resto_campos.contains("D/") {
        let parts: Vec<&str> = resto_campos.splitn(2, "D/").collect();
        if parts.len() == 2 {
            let quantity_raw = format!("D/{}", parts[1].trim());
            let quantity_cortada = cortar_quantity(&quantity_raw);
            (parts[0].trim().to_string(), quantity_cortada)
        } else {
            (resto_campos.clone(), String::new())
        }
    } else if resto_campos.contains("E/") {
        let parts: Vec<&str> = resto_campos.splitn(2, "E/").collect();
        if parts.len() == 2 {
            let quantity_raw = format!("E/{}", parts[1].trim());
            let quantity_cortada = cortar_quantity(&quantity_raw);
            (parts[0].trim().to_string(), quantity_cortada)
        } else {
            (resto_campos.clone(), String::new())
        }
    } else if resto_campos.contains("D./") {
        let parts: Vec<&str> = resto_campos.splitn(2, "D./").collect();
        if parts.len() == 2 {
            let quantity_raw = format!("D./{}", parts[1].trim());
            let quantity_cortada = cortar_quantity(&quantity_raw);
            (parts[0].trim().to_string(), quantity_cortada)
        } else {
            (resto_campos.clone(), String::new())
        }
    } else {
        // Si no hay D/, E/ o D./, todo es operador_cargo
        (resto_campos, String::new())
    };

    Some(Nave {
        eta,
        ship_name,
        etb,
        tipo,
        mes_actual: mes.to_string(),
        operador_y_cargo: operador_cargo,
        quantity,
    })
}
