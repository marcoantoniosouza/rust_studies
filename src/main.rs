use calamine::{Reader, Xlsx, open_workbook, DataType};
use chrono::{Duration, NaiveDate};

fn main() {
    let path: String = String::from("/home/marcoantonio/github/rust_studies/src/file.xlsx");
    let mut excel: Xlsx<_> = open_workbook(path)
        .expect("ERROR OPENING THE FILE");

    if let Some(Ok(r)) =
        excel.worksheet_range(excel
            .sheet_names()
            .first()
            .unwrap()) {

        for row in r.rows() {
            println!("--------------------");
            for cell in row {
                let str1 = get_value_as_string(cell.clone());
                println!("{str1}");
            }
        }
    }
}

fn get_value_as_string(data: DataType) -> String {
    data
        .as_string()
        .unwrap_or(get_date_as_string(data.to_string().parse::<i64>().unwrap_or(0)))
}
fn get_date(data: i64) -> NaiveDate {
    let start = NaiveDate::from_ymd_opt(1899, 12, 30).expect("DATE");
    let date = start.checked_add_signed(Duration::days(data));
    date.unwrap()
}

fn get_date_as_string(data: i64) -> String {
    get_date(data).to_string()
}
