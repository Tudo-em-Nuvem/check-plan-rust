use excelize::{Cell, Spreadsheet};
use polars::prelude::*;
use regex::Regex;

const NAME_SHEET_OMIE: &str = "Contratos por Cliente";

#[derive(Debug)]
pub struct Column {
    index: u32,
    itens: Vec<String>,
}

fn main() {
    let column_cnum_cobranca = get_cnum_cobrancas();
    let column_cnum_omie = get_cnums_omie();

    println!("Primeiros 6 itens de column_cnum_omie:");
    for item in column_cnum_omie.itens.iter().take(6) {
        println!("{:?}", item);
    }

    println!("Primeiros 6 itens de column_cnum_cobranca:");
    for item in column_cnum_cobranca.itens.iter().take(6) {
        println!("{:?}", item);
    }
}

fn get_cnums_omie() -> Column {
    let mut column_cnum_omie = Column {
        index: 5,
        itens: Vec::new(),
    };
    let column_qtd_omie = Column {
        index: 8,
        itens: Vec::new(),
    };

    let ws = Spreadsheet::open_file(String::from("planilhas/pivot.xlsx")).unwrap();
    let mut finish = false;
    let mut counter = 3;

    while !finish {
        let cell = ws
            .get_cell_value(NAME_SHEET_OMIE, counter, column_qtd_omie.index)
            .unwrap();

        if cell.is_empty() {
            finish = true
        }

        let cnumctr = ws
            .get_cell_value(NAME_SHEET_OMIE, counter, column_cnum_omie.index)
            .unwrap();

        if !cnumctr.is_empty() {
            column_cnum_omie.itens.push(cnumctr);
        }
        counter += 1;
    }

    column_cnum_omie
}

fn get_cnum_cobrancas() -> Column {
    let mut column_cnum_asaas = Column {
        index: 3,
        itens: Vec::new(),
    };

    let df_cobrancas = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("planilhas/cobrancas.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut finish = false;
    let mut count = 2;
    let re = Regex::new(r"(\d{4}/\d{5})").unwrap();

    while !finish {
        match df_cobrancas.column("Nome").unwrap().get(count) {
            Ok(item) => {
                for cap in re.captures_iter(&item.to_string()) {
                    column_cnum_asaas.itens.push(cap[0].to_string());
                }
                count += 1;
            }
            Err(_e) => finish = true,
        }
    }

    column_cnum_asaas
}
