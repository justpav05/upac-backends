# upac-backends

> ⚠️ Проект находится в активной разработке. API нестабильно.

**upac** — универсальный менеджер пакетов для Linux. Позволяет устанавливать, удалять и управлять пакетами различных форматов из единого инструмента, не привязываясь к конкретному дистрибутиву. Опционально интегрируется с [OSTree](https://ostreedev.github.io/ostree/) для атомарных снапшотов состояния системы — это позволяет откатить систему к любому предыдущему состоянию если что-то пошло не так.

---

## Возможности

- Установка и удаление пакетов из различных форматов
- Реестр установленных пакетов и их файлов
- Поддержка нескольких форматов пакетов через систему бэкендов
- Интеграция с OSTree: создание коммитов, откат системы к предыдущему состоянию
- Файловые блокировки для безопасной параллельной работы

---

## Доступность

| Пакетный менеджер | Статус |
|---|---|
| crates.io | 🔜 Скоро |
| AUR (Arch Linux) | 🔜 Скоро |
| APT (Debian/Ubuntu) | 🔜 Скоро |

---

## upac-backends

Данный репозиторий содержит **upac-backends** — набор бэкендов для работы с различными форматами пакетов. Каждый бэкенд реализует трейт `Backend` из [upac-core-lib](https://github.com/justpav05/upac-core-lib) и отвечает за извлечение файлов и чтение метаданных из пакетов конкретного формата.

### Доступные бэкенды

| Бэкенд | Крейт | Форматы | Статус |
|---|---|---|---|
| ALPM (Arch Linux) | `upac-backend-alpm` | `.pkg.tar.zst`, `.pkg.tar.xz`, `.pkg.tar.gz` | ✅ Реализован |

### Использование

Добавьте нужный бэкенд в зависимости:

```toml
[dependencies]
upac-backend-alpm = { git = "https://github.com/justpav05/upac-backends.git", branch = "main" }
```

Затем передайте его в установщик:

```rust
use upac_backend_alpm::AlpmBackend;
use upac_core_lib::Backend;

let backend = AlpmBackend;

// Определить формат пакета
if backend.detect(Path::new("firefox-120.0-1-x86_64.pkg.tar.zst")) {
    // Прочитать метаданные
    let metadata = backend.read_metadata(path)?;

    // Извлечь файлы во временную директорию
    let extracted = backend.extract(path, &temp_dir)?;
}
```

### Реализация собственного бэкенда

Для поддержки нового формата достаточно реализовать трейт `Backend` из `upac-core-lib`:

```rust
use upac_core_lib::{Backend, ExtractedPackage, PackageMetadata};
use std::path::Path;

pub struct MyBackend;

impl Backend for MyBackend {
    fn name(&self) -> &str { "my-backend" }

    fn supported_formats(&self) -> Vec<&str> {
        vec!["my.pkg"]
    }

    fn detect(&self, path: &Path) -> bool {
        path.to_string_lossy().ends_with(".my.pkg")
    }

    fn extract(&self, path: &Path, temp_dir: &Path) -> upac_core_lib::backend::Result<ExtractedPackage> {
        todo!()
    }

    fn read_metadata(&self, path: &Path) -> upac_core_lib::backend::Result<PackageMetadata> {
        todo!()
    }
}
```

### Требования

- Rust 1.83+
- Linux
- [upac-core-lib](https://github.com/justpav05/upac-core-lib)

### Зависимости

| Крейт | Назначение |
|---|---|
| `upac-core-lib` | Трейт `Backend` и общие типы |
| `nix` | Unix: права доступа |
| `zstd` | Распаковка `.pkg.tar.zst` |
| `flate2` | Распаковка `.pkg.tar.gz` |
| `xz2` | Распаковка `.pkg.tar.xz` |
| `tar` | Работа с tar-архивами |
