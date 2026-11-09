mod checker;
mod nix;

use colored::*;

fn main() {
    // Стильный заголовок в терминале
    println!();
    println!("{}", "  NIX-HEALTH AUDITOR v0.1.0 ".on_cyan().black().bold());
    println!("{}", "  [System Integrity & Store Inspector]".cyan().italic());
    println!();

    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let config_path = format!("{}/.config", home);

    // 1. Проверка симлинков
    println!("{} Сканирование конфигураций в ~/.config...", "[1/2]".yellow().bold());
    let broken_links = checker::scan_broken_symlinks(&config_path, &home);

    if broken_links.is_empty() {
        println!("  {}: Все симлинки корректны, мёртвых ссылок не обнаружено.", "SUCCESS".green().bold());
    } else {
        println!("  {}: Обнаружено битых симлинков: {}", "WARNING".red().bold(), broken_links.len());
        
        let mut table = comfy_table::Table::new();
        table.load_preset(comfy_table::presets::UTF8_FULL);
        table.set_header(vec!["Битый симлинк (Конфиг)", "Указывает на несуществующий путь"]);

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

    // 2. Анализ /nix/store
    println!("{} Анализ состояния /nix/store...", "[2/2]".yellow().bold());
    match nix::get_store_stats() {
        Ok((count, total_bytes)) => {
            let gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            println!("  • Зарегистрированных путей: {}", count.to_string().cyan());
            println!("  • Общий объем в сторе:       {:.2} GB", format!("{:.2}", gb).green().bold());
        }
        Err(e) => {
            println!("  {}: {}", "SKIP".dimmed(), e.dimmed());
        }
    }

    println!();
    println!("{}", " Аудит успешно завершён. Система не затронута (Read-Only).".dimmed());
    println!();
}