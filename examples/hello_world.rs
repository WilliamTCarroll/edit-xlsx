use edit_xlsx::{Format, FormatAlignType, FormatBorderType, FormatColor, Workbook, WorkbookResult, WorkSheet, Write};

fn main() -> WorkbookResult<()> {
    // Create a new workbook
    let mut workbook = Workbook::new();
    let worksheet = workbook.get_worksheet_mut(1)?;
    // write some text
    WorkSheet::write(worksheet, "A1", "Hello")?;
    worksheet.write("B1", "World")?;
    worksheet.write("C1", "Rust")?;
    // Adjust font size
    let big = Format::default().set_size(32);
    worksheet.write_with_format("B1", "big text", &big)?;
    // Change font color
    let red = Format::default().set_color(FormatColor::RGB(255, 119, 119));
    worksheet.write_with_format("C1", "red text", &red)?;
    // Change the font style
    let bold = red.set_bold();
    worksheet.write_with_format("D1", "red bold text", &bold)?;
    // Change font
    let font = Format::default().set_font("华文行楷");
    worksheet.write_with_format("E1", "你好", &font)?;
    // adjust the text align
    let left_top = Format::default().set_align(FormatAlignType::Left).set_align(FormatAlignType::Top);
    worksheet.write_with_format("A2", "left top", &left_top)?;
    // add borders
    let thin_border = Format::default().set_border(FormatBorderType::Thin);
    worksheet.write_with_format("B2", "bordered text", &thin_border)?;
    // add background
    let red_background = Format::default().set_background_color(FormatColor::RGB(255, 119, 119));
    worksheet.write_with_format("C2", "red", &red_background)?;
    // add a number
    worksheet.write("D2", std::f64::consts::PI)?;
    // add a new worksheet and set a tab color
    let worksheet = workbook.add_worksheet_by_name("Other examples")?;
    worksheet.set_tab_color(&FormatColor::RGB(255, 153, 0)); // Orange
    // Set a background.
    worksheet.set_background("examples/pics/ferris.png")?;
    // Create a format to use in the merged range.
    let merge_format = Format::default()
        .set_bold()
        .set_border(FormatBorderType::Double)
        .set_align(FormatAlignType::Center)
        .set_align(FormatAlignType::VerticalCenter)
        .set_background_color(FormatColor::RGB(255, 255, 0));
    // Merge cells.
    worksheet.merge_range_with_format("A1:C3", "Merged Range", &merge_format)?;
    // Add an image
    worksheet.insert_image("A4:C10", &"./examples/pics/rust.png")?;
    workbook.save_as("examples/hello_world.xlsx")?;
    Ok(())
}