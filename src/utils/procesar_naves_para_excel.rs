use crate::{model::{Nave, NaveExcel, Agente}, utils::{fechas::convertir_eta_etb_inteligente, leer_agentes::buscar_agente_por_operador}};

pub fn procesar_naves_para_excel(naves: &[Nave], agentes: &[Agente]) -> Result<Vec<NaveExcel>, Box<dyn std::error::Error>> {
    
    let mut naves_excel = Vec::new();

    for nave in naves {
        // Solo procesar naves con tipo GENERAL, GRANELERO, RORO
        let tipo_carga = if nave.tipo.contains("GENERAL") {
            "GENERAL".to_string()
        } else if nave.tipo.contains("GRANELERO") {
            "GRANELERO".to_string()
        } else if nave.tipo.contains("RORO") {
            "RORO".to_string()
        } else {
            continue; // Saltar si no es uno de los tipos deseados
        };

        // Saltar naves sin mes definido o con mes inválido
        if nave.mes_actual.is_empty() || nave.mes_actual == "DESCONOCIDO" {
            println!("Saltando nave '{}' por mes inválido: '{}'", nave.ship_name, nave.mes_actual);
            continue;
        }

        // Convertir ETA y ETB a fechas usando la lógica inteligente
        let (eta_date, etb_date) = match convertir_eta_etb_inteligente(&nave.eta, &nave.etb, &nave.mes_actual) {
            Ok((eta, etb)) => (eta, etb),
            Err(_) => {
                continue;
            }
        };

        // Calcular diferencia entre ETB y ETA en días
        let dias_en_bahia = (etb_date - eta_date).num_days();

        if dias_en_bahia < 5 {
            continue;
        }

        // Buscar información del agente basado en el operador
        let agente_info = buscar_agente_por_operador(agentes, &nave.operador_y_cargo);

        let nave_excel = NaveExcel {
            nave: nave.ship_name.clone(),
            tipo_carga,
            eta_formatted: eta_date.format("%d-%m-%Y %H:%M").to_string(),
            etb_formatted: etb_date.format("%d-%m-%Y %H:%M").to_string(),
            dias_en_bahia,
            operador_y_cargo: nave.operador_y_cargo.clone(),
            quantity: nave.quantity.clone(),
            operador: agente_info.map(|a| a.operador.clone()),
            cargo: agente_info.map(|a| a.cargo.clone()),
            nombre_apellido: agente_info.map(|a| a.nombre_apellido.clone()),
            telefono: agente_info.map(|a| a.telefono.clone()),
            correo: agente_info.map(|a| a.correo.clone()),
        };

        naves_excel.push(nave_excel);
    }

    Ok(naves_excel)
}