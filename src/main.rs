use bytefmt::{ByteSize, ParseError, UnitSystem};
use std::process;

fn main() {
    if let Err(error) = run() {
        eprintln!("bytefmt: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            let input = args.get(2).ok_or_else(|| "missing input".to_string())?;
            let size = ByteSize::parse(input).map_err(|error| error.to_string())?;
            println!("{}", size.0);
        }
        "format" => {
            let bytes_text = args.get(2).ok_or_else(|| "missing byte count".to_string())?;
            let bytes: u64 = bytes_text
                .parse()
                .map_err(|_| "invalid byte count".to_string())?;
            let system = parse_system(get_option(&args, "--unit").as_deref().unwrap_or("binary"))?;
            let precision: usize = get_option(&args, "--precision")
                .unwrap_or_else(|| "2".to_string())
                .parse()
                .map_err(|_| "invalid precision".to_string())?;
            println!("{}", ByteSize(bytes).format(system, precision));
        }
        "convert" => {
            let input = args.get(2).ok_or_else(|| "missing input".to_string())?;
            let size = ByteSize::parse(input).map_err(|error| error.to_string())?;
            let system = parse_system(
                get_option(&args, "--to")
                    .ok_or_else(|| "missing --to unit system".to_string())?
                    .as_str(),
            )?;
            let precision: usize = get_option(&args, "--precision")
                .unwrap_or_else(|| "2".to_string())
                .parse()
                .map_err(|_| "invalid precision".to_string())?;
            println!("{}", size.format(system, precision));
        }
        "help" | "-h" | "--help" => print_usage(),
        _ => return Err(format!("unknown command: {}", args[1])),
    }

    Ok(())
}

fn get_option(args: &[String], name: &str) -> Option<String> {
    for (index, arg) in args.iter().enumerate() {
        if arg == name {
            return args.get(index + 1).cloned();
        }
        if let Some(value) = arg.strip_prefix(&format!("{name}=")) {
            return Some(value.to_string());
        }
    }
    None
}

fn parse_system(value: &str) -> Result<UnitSystem, String> {
    match value.to_ascii_lowercase().as_str() {
        "binary" | "bin" => Ok(UnitSystem::Binary),
        "decimal" | "dec" => Ok(UnitSystem::Decimal),
        _ => Err(format!("unknown unit system: {value}")),
    }
}

fn print_usage() {
    println!("bytefmt - parse, format, and convert byte sizes");
    println!();
    println!("Usage:");
    println!("  bytefmt parse <INPUT>");
    println!("  bytefmt format <BYTES> [--unit binary|decimal] [--precision N]");
    println!("  bytefmt convert <INPUT> --to binary|decimal [--precision N]");
    println!("  bytefmt help");
    println!();
    println!("Examples:");
    println!("  bytefmt parse \"1.5 MiB\"");
    println!("  bytefmt format 1536 --unit binary");
    println!("  bytefmt convert \"1.5 MB\" --to binary");
}
