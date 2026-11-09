use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Deserialize, Debug)]
pub struct PathInfo {
    #[serde(rename = "narSize")]
    pub nar_size: Option<u64>,
}

/// Получает статистику по путям в /nix/store через официальный JSON API Nix
pub fn get_store_stats() -> Result<(usize, u64), String> {
    let output = Command::new("nix")
        .args(["path-info", "--all", "--json"])
        .output()
        .map_err(|e| format!("Не удалось запустить команду 'nix'. Ошибка: {}", e))?;

    if !output.status.success() {
        return Err("Команда 'nix path-info' завершилась с кодом ошибки.".to_string());
    }

    let parsed: HashMap<String, PathInfo> = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Ошибка парсинга JSON ответа Nix: {}", e))?;

    let count = parsed.len();
    let total_bytes: u64 = parsed.values().map(|info| info.nar_size.unwrap_or(0)).sum();

    Ok((count, total_bytes))
}