use crate::model::NaveExcel;
use rust_xlsxwriter::*;

pub fn exportar_a_excel(naves: &[NaveExcel]) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();

    // Crear formatos
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::Gray)
        .set_align(FormatAlign::Center);

    let number_format = Format::new()
        .set_align(FormatAlign::Center);

    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Naves")?;

    // Escribir encabezados
    let headers = vec![
        "Nave",
        "Tipo de Carga",
        "ETA",
        "ETB",
        "Días en Bahía",
        "Operador y Cargo",
        "Cantidad",
        "Operador",
        "Cargo",
        "Nombre y Apellido",
        "Teléfono",
        "Correo"
    ];

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, &header_format)?;
    }

    // Escribir datos
    for (row, nave) in naves.iter().enumerate() {
        let row = (row + 1) as u32;

        worksheet.write_string(row, 0, &nave.nave)?;
        worksheet.write_string(row, 1, &nave.tipo_carga)?;
        worksheet.write_string(row, 2, &nave.eta_formatted)?;
        worksheet.write_string(row, 3, &nave.etb_formatted)?;
        worksheet.write_number_with_format(row, 4, nave.dias_en_bahia as f64, &number_format)?;
        worksheet.write_string(row, 5, &nave.operador_y_cargo)?;
        worksheet.write_string(row, 6, &nave.quantity)?;
        worksheet.write_string(row, 7, nave.operador.as_deref().unwrap_or(""))?;
        worksheet.write_string(row, 8, nave.cargo.as_deref().unwrap_or(""))?;
        worksheet.write_string(row, 9, nave.nombre_apellido.as_deref().unwrap_or(""))?;
        worksheet.write_string(row, 10, nave.telefono.as_deref().unwrap_or(""))?;
        worksheet.write_string(row, 11, nave.correo.as_deref().unwrap_or(""))?;
    }

    // Ajustar ancho de columnas
    worksheet.set_column_width(0, 30.0)?; // Nave
    worksheet.set_column_width(1, 15.0)?; // Tipo de Carga
    worksheet.set_column_width(2, 20.0)?; // ETA
    worksheet.set_column_width(3, 20.0)?; // ETB
    worksheet.set_column_width(4, 15.0)?; // Días en Bahía
    worksheet.set_column_width(5, 50.0)?; // Operador y Cargo
    worksheet.set_column_width(6, 15.0)?; // Cantidad
    worksheet.set_column_width(7, 20.0)?; // Operador
    worksheet.set_column_width(8, 20.0)?; // Cargo
    worksheet.set_column_width(9, 25.0)?; // Nombre y Apellido
    worksheet.set_column_width(10, 15.0)?; // Teléfono
    worksheet.set_column_width(11, 30.0)?; // Correo

    // Generar nombre de archivo con fecha y hora actual
    let fecha_hora = chrono::Local::now().format("%Y%m%d_%H%M");
    let nombre_archivo = format!("naves_reporte_{}.xlsx", fecha_hora);
    
    workbook.save(&nombre_archivo)?;
    Ok(())
}

