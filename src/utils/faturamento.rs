use polars::prelude::*;
use excelize::{Cell, Spreadsheet};
use crate::utils::extract_domain::extract_domain;

#[derive(Debug, PartialEq)]
enum Columns {
    Domain,
    Fat,
    Cnum,
    Desc,
    Qtd,
    Situacao
}

#[derive(Debug)]
pub struct Column {
    name: Columns,
    index: u32,
    itens: Vec<String>
}

impl Column {
    fn new(name: Columns, index: u32) -> Self {
        Self { name, index, itens: Vec::new() }
    }
}

struct Omie {
    dominio: String,
    produto:String,
    ativas: i8,
    arquivadas: i8,
    cloud_identity: String,
    app_sheet: String,
    vault: String,
    nao_mensais: bool,
    status: String,
    dia_faturamento: i8
}

pub(crate) fn gerar_planilha_faturamento () {
    let df_google = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("planilhas/teste.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

    // csv google
    println!("{:?}", df_google[0].get(0).unwrap());

    generate_omie();
}

fn generate_omie () -> DataFrame {
    let mut column_domain_omie = Column::new(Columns::Domain, 1);
    let mut column_cnum_omie = Column::new(Columns::Cnum, 2);
    let mut column_situacao_omie = Column::new(Columns::Situacao, 3);
    let mut column_dia_fat_omie = Column ::new(Columns::Fat, 4);
    let mut column_desc_omie = Column::new(Columns::Desc,5);
    let mut column_quantidade_omie = Column::new(Columns::Qtd, 7);


    let mut columns: Vec<&mut Column> = Vec::new();
    columns.push(&mut column_desc_omie);
    columns.push(&mut column_domain_omie);
    columns.push(&mut column_dia_fat_omie);
    columns.push(&mut column_cnum_omie);
    columns.push(&mut column_quantidade_omie);
    columns.push(&mut column_situacao_omie);

    let wb = Spreadsheet::open_file(String::from("planilhas/teste.xlsx"));
    match wb {
         Ok(ws) => {
            let mut final_x: u32 = 0;
            
            for column in columns {
                if column.name == Columns::Desc {
                    let mut counter = 3;
                    while counter != final_x {
                        match ws.get_cell_value("Contratos por Cliente", counter, column.index) {
                            Ok(c) => {
                                let cell = String::from(c);
                                if cell.is_empty() { 
                                    final_x = counter;
                                    break;
                                 } else {
                                    column.itens.push(cell);
                                 }
                            } Err(e) => println!("{:?}", e)
                        }

                        counter+=1
                    }
                } else {
                    for i in 3..final_x {
                        match ws.get_cell_value("Contratos por Cliente", i, column.index) {
                            Ok(c) => column.itens.push(String::from(c)),
                            Err(e) => println!("{:?}", e)
                        } 
                    }
                }
            }
        } Err(e) => print!("{:?}", e),
    }
    
    let data: DataFrame = df!(
        "Domain" => column_domain_omie.itens,
        "Dia Faturamento" => column_dia_fat_omie.itens,
        "Cnum" => column_cnum_omie.itens,
        "Descrição" => column_desc_omie.itens,
        "Quantidade" => column_quantidade_omie.itens,
        "Situação" => column_situacao_omie.itens
    ).unwrap();
    println!("{:?}", data);

    let mut cliente_atual = String::new();
    let mut status_atual = String::new();
    let mut ultimo_cliente = String::new();
    let mut rodada_inicial = true;
    let mut dia_atual: i8 = 0;
    let mut ultimo_cliente_tratado = String::new();
    
    let mut clientes_nao_divergentes = Vec::new();
    let mut clientes_divergentes = Vec::new();

    for index_x in 0..data.height() {
        let row = data.get_row(index_x).unwrap();
        println!("{:?}", row.0.get(index_x).unwrap());
        
        let cliente = row.0.get(0).unwrap().to_string();
        let status = row.0.get(5).unwrap().to_string();
        let dia = row.0.get(4).unwrap().to_string().parse::<i8>().unwrap();
        let descricao = row.0.get(3).unwrap().to_string();

        if !ultimo_cliente.is_empty() {
            rodada_inicial = true;
            cliente_atual  = cliente;
            status_atual   = status;
            dia_atual      = dia;
        }

        let mut domain = String::new();

        match extract_domain(&descricao) {
            Ok(d) => domain = d,
            Err(_e) => {
                if ultimo_cliente.contains(&cliente_atual)  {
                    domain = ultimo_cliente;
                } else if rodada_inicial {
                    domain = cliente_atual.clone();
                } else {
                    domain = format!("Cliente não encontrado.\nultimo tratado: {ultimo_cliente_tratado}\nobs: {descricao}\ncliente coluna: {cliente_atual}");
                }
            }
        }

        if descricao.to_lowercase().contains("microsoft") {
            if clientes_nao_divergentes.iter().find(|&&x| x == cliente_atual).is_none() {
                clientes_nao_divergentes.push(cliente_atual.clone());
            }
        }

        ultimo_cliente = "algo".to_string();
    }

    data
}
