pub mod vga_color;
pub mod vga_pac;
pub mod vga_palette;

use std::io::Write;

use clap::ValueEnum;

use self::vga_pac::VgaPac;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VgaPacParseMode {
    PaletteGimp,
    PaletteNeopaint,
    CreateMaterials,
}

pub fn parse_vga_pac_file(
    file: &str,
    _mode: &VgaPacParseMode,
    _output_file: &Option<String>,
) -> Result<(), String> {
    let vga_pac =
        VgaPac::read_from_file(file).map_err(|e| format!("Error reading VGA.PAC file: {e}"))?;

    vga_pac.debug_print_gimp_palette();

    // let writer = match output_file {
    //     Some(output_file) => {
    //         let file = File::open(file).map_err(|e| format!("Unable to open file {file}: {e}"))?;

    //         // let mut reader = BufReader::new(file);

    //         BufWriter::new(file)
    //     }
    //     None => std::io::stdout(),
    // };

    Ok(())
}

fn _work(writer: &mut dyn Write, _vga_pac: VgaPac) -> std::io::Result<()> {
    // writer.wri
    write!(writer, "asdasdf")?;

    Ok(())
}
