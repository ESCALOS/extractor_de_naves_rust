use chrono::{DateTime, Local, NaiveDate, TimeZone};

pub fn obtener_mes_anterior(mes: &str) -> String {
    match mes.trim().to_uppercase().as_str() {
        "ENERO" => "DICIEMBRE".to_string(),
        "FEBRERO" => "ENERO".to_string(),
        "MARZO" => "FEBRERO".to_string(),
        "ABRIL" => "MARZO".to_string(),
        "MAYO" => "ABRIL".to_string(),
        "JUNIO" => "MAYO".to_string(),
        "JULIO" => "JUNIO".to_string(),
        "AGOSTO" => "JULIO".to_string(),
        "SEPTIEMBRE" => "AGOSTO".to_string(),
        "OCTUBRE" => "SEPTIEMBRE".to_string(),
        "NOVIEMBRE" => "OCTUBRE".to_string(),
        "DICIEMBRE" => "NOVIEMBRE".to_string(),
        _ => "DESCONOCIDO".to_string(),
    }
}

pub fn convertir_eta_etb_a_fecha(eta_etb: &str, mes: &str) -> Result<DateTime<Local>, Box<dyn std::error::Error>> {
    // ETA/ETB formato: ddHHmm (ejemplo: 021000 = día 02, hora 10, minuto 00)
    if eta_etb.len() != 6 {
        return Err(format!("Formato de ETA/ETB inválido: '{}' (longitud: {})", eta_etb, eta_etb.len()).into());
    }

    let dia = &eta_etb[0..2].parse::<u32>()?;
    let hora = &eta_etb[2..4].parse::<u32>()?;
    let minuto = &eta_etb[4..6].parse::<u32>()?;

    // println!("DEBUG: Convirtiendo ETA/ETB: {} -> día: {}, hora: {}, minuto: {}, mes: '{}'",
    //          eta_etb, dia, hora, minuto, mes);

    // Extraer solo el nombre del mes (primera palabra) para manejar casos como "DICIEMBRE 00 1"
    let mes_limpio = mes.split_whitespace().next().unwrap_or("").trim().to_uppercase();

    // Obtener el número del mes
    let mes_num = match mes_limpio.as_str() {
        "ENERO" => 1,
        "FEBRERO" => 2,
        "MARZO" => 3,
        "ABRIL" => 4,
        "MAYO" => 5,
        "JUNIO" => 6,
        "JULIO" => 7,
        "AGOSTO" => 8,
        "SEPTIEMBRE" => 9,
        "OCTUBRE" => 10,
        "NOVIEMBRE" => 11,
        "DICIEMBRE" => 12,
        _ => return Err(format!("Mes inválido: '{}' (limpio: '{}')", mes, mes_limpio).into()),
    };

    // Asumir año actual (2025)
    let año = 2025;

    let fecha = NaiveDate::from_ymd_opt(año, mes_num, *dia)
        .ok_or("Fecha inválida")?
        .and_hms_opt(*hora, *minuto, 0)
        .ok_or("Hora inválida")?;

    Ok(Local.from_local_datetime(&fecha).single().ok_or("Error de conversión de zona horaria")?)
}

pub fn convertir_eta_etb_inteligente(eta: &str, etb: &str, mes_base: &str) -> Result<(DateTime<Local>, DateTime<Local>), Box<dyn std::error::Error>> {
    // Extraer días de ETA y ETB
    let dia_eta = eta[0..2].parse::<u32>()?;
    let dia_etb = etb[0..2].parse::<u32>()?;

    // Si el día del ETB es menor que el del ETA, significa que ETB está en el mes siguiente
    let (mes_eta, mes_etb) = if dia_etb < dia_eta {
        // ETB está en el mes siguiente
        let mes_anterior = obtener_mes_anterior(mes_base);
        (mes_anterior, mes_base.to_string())
    } else {
        // Ambos están en el mismo mes
        (mes_base.to_string(), mes_base.to_string())
    };

    // println!("DEBUG: ETA día {} (mes: {}), ETB día {} (mes: {})", dia_eta, mes_eta, dia_etb, mes_etb);

    let eta_date = convertir_eta_etb_a_fecha(eta, &mes_eta)?;
    let etb_date = convertir_eta_etb_a_fecha(etb, &mes_etb)?;

    Ok((eta_date, etb_date))
}
