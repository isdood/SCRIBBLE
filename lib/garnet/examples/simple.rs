use garnet::{Terminal, GarnetConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create terminal configuration
    let config = GarnetConfig {
        width: 80,
        height: 25,
        crystal_freq: 60,
        dream_buffer_size: 4096,
    };

    // Initialize terminal
    let mut terminal = Terminal::new(config)?;

    // Test output
    terminal.write_char('H')?;
    terminal.write_char('e')?;
    terminal.write_char('l')?;
    terminal.write_char('l')?;
    terminal.write_char('o')?;
    terminal.write_char('\n')?;
    terminal.write_char('W')?;
    terminal.write_char('o')?;
    terminal.write_char('r')?;
    terminal.write_char('l')?;
    terminal.write_char('d')?;
    terminal.write_char('!')?;

    Ok(())
}
