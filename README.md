Some Rust code examples

```Rust

// read Excel file
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};

fn main() -> Result<(), Error> {
    // Open the XLSX file
    let mut workbook: Xlsx<_> = open_workbook("path/to/your/file.xlsx")?; 

    // Get the first worksheet (adjust sheet name if needed)
    let range = workbook.worksheet_range("Sheet1")?; 

    // Create a deserializer
    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    // Iterate over rows and deserialize data
    for result in iter {
        // Process the row 
        let row: Vec<String> = result?; 
        println!("{:?}", row); 
    }

    Ok(())
}

//------------------------

// https://crates.io/crates/rust_xlsxwriter
// https://rustxlsxwriter.github.io/getting_started.html
// cargo add rust_xlsxwriter

// write  Excel file
use rust_xlsxwriter::*;

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file
    let mut workbook = Workbook::new();
    let mut worksheet = workbook.add_worksheet(); // workbook.add_worksheet().set_name("Pane 1")?;

    // Create a bold format
    let bold = Format::new().set_bold();
    let decimal_format = Format::new().set_num_format("0.000"); //set_num_format("#,##0.00");
    let date_format = Format::new().set_num_format("yyyy-mm-dd");

    worksheet.set_column_width(0, 22)?;
    // Write some data
    worksheet.write(0, 0, "A")?;
    worksheet.write(0, 1, "B")?;
    worksheet.write_with_format(0, 2, "C", &bold)?;
    worksheet.write(1, 0, 10.50)?;
    worksheet.write_with_format(1, 1, 20.123,&decimal_format)?;

    let date = ExcelDateTime::from_ymd(2023, 1, 25)?;
    worksheet.write_with_format(2, 0, &date, &date_format)?;

    // Write the formula in cell C1
    worksheet.write(1, 2, Formula::new("=A1+B1"))?; 

    workbook.save("demo.xlsx")?;
    // let buf : Vec<u8>  = workbook.save_to_buffer()?;  // to stream buffer

    Ok(())
}

```

