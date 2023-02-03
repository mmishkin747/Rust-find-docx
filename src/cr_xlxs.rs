use calamine::{Reader, Xlsx, open_workbook, DataType};

use crate::MyResult;


pub fn open_xlsx(filename: &str) -> MyResult<String> {
    let mut excel: Xlsx<_> = open_workbook(filename)?;
    let mut res = filename.to_string();
    res.push_str("\n");
    for (name, _) in excel.worksheets(){
        
        if let Some(Ok(r)) = excel.worksheet_range(&name) {
            res.push_str(name.as_str());
            res.push_str("\n");
            for row in r.rows() {

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

    }
    };
    Ok(res)

}