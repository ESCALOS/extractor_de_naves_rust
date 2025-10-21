pub struct NaveExcel {
    pub nave: String,
    pub tipo_carga: String,
    pub eta_formatted: String,
    pub etb_formatted: String,
    pub dias_en_bahia: i64,
    pub operador_y_cargo: String,
    pub quantity: String,
    pub operador: Option<String>,
    pub cargo: Option<String>,
    pub nombre_apellido: Option<String>,
    pub telefono: Option<String>,
    pub correo: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Agente {
    pub operador: String,
    pub cargo: String,
    pub nombre_apellido: String,
    pub telefono: String,
    pub correo: String,
}

pub struct Nave {
    pub eta: String,
    pub ship_name: String,
    pub etb: String,
    pub tipo: String,
    pub mes_actual: String,
    pub operador_y_cargo: String,
    pub quantity: String,
}
