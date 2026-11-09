mod checker;
mod nix;

use colored::*;

fn print_header() {
    println!();
    println!(
        "{} {} {}",
        "::".truecolor(100, 150, 255),
        "nix-health".bold().white(),
        "v0.1.0 — NixOS Environment Inspector".truecolor(140, 140, 140)
    );
    println!(
        "{}",
        "─────────────────────────────────────────────────────────────────".truecolor(60, 60, 60)
    );
    println!();
}

fn main() {
    print_header();

    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let config_path = format!("{}/.config", home);

    // 1. Scan symlinks
    println!("{} Scanning configurations in ~/.config...", "[1/2]".yellow().bold());
    let broken_links = checker::scan_broken_symlinks(&config_path, &home);

    if broken_links.is_empty() {
        println!("  {}: All symlinks are valid. No broken references found.", "SUCCESS".green().bold());
    } else {
        println!("  {}: Found broken symlinks: {}", "WARNING".red().bold(), broken_links.len());
        
        let mut table = comfy_table::Table::new();
        table.load_preset(comfy_table::presets::UTF8_FULL);
        table.set_header(vec!["Broken Symlink (Config)", "Points to Non-existent Path"]);

        for link in broken_links.iter().take(15) {
            table.add_row(vec![
                checker::sanitize_path(&link.path, &home),
                checker::sanitize_path(&link.target, &home),
            ]);
        }
        println!();
        println!("{table}");
    }

    println!();

    // 2. Analyze /nix/store
    println!("{} Analyzing /nix/store state...", "[2/2]".yellow().bold());
    match nix::get_store_stats() {
        Ok((count, total_bytes)) => {
            let gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            println!("  • Registered store paths: {}", count.to_string().cyan());
            println!("  • Total store size:        {:.2} GB", gb);
        }
        Err(e) => {
            println!("  {}: {}", "SKIP".dimmed(), e.dimmed());
        }
    }

    println!();
    println!("{}", " Audit completed successfully. System state unchanged (Read-Only).".dimmed());
    println!();
}