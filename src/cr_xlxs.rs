use calamine::{Reader, Xlsx, open_workbook, DataType};

use crate::MyResult;


pub fn open_xlsx(filename: &str) -> MyResult<String> {
    let mut excel: Xlsx<_> = open_workbook(filename)?;
    let mut res = String::new();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {

            //println!("row={:?}", row,);
            for cell in row {
                match cell {
                    DataType::String(_) => {if let Some(value) = cell.get_string(){
                        res.push_str(value);
                        res.push_str("  ");
                    }},
                    DataType::Float(_) => {if let Some(value) = cell.get_float() {
                        res.push_str(value.to_string().as_str());
                        res.push_str("  ");
                    }}
                    _ => {}
                }

            }
            res.push_str("\n");
        }
    };
    Ok(res)

}