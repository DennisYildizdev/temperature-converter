# Temperature Converter

Rust ile yazılmış, Celsius ve Fahrenheit arasında dönüşüm yapan küçük bir komut satırı uygulaması.

## Çalıştırma

Rust 2024 edition destekleyen Rust 1.85 veya daha yeni bir sürüm gerekir.

```sh
git clone https://github.com/DennisYildizdev/temperature-converter.git
cd temperature-converter
cargo run
```

Önce sıcaklığı, sonra **girdi sıcaklığının birimini** yaz:

- `c`: Celsius → Fahrenheit
- `f`: Fahrenheit → Celsius

Örnek: `100` ve ardından `c` girildiğinde sonuç `212` (°F) olur. `32` ve ardından `f` girildiğinde sonuç `0` (°C) olur.

Ondalık sayılarda nokta kullan (`36.5`). Mevcut sürüm yalnızca küçük harf `c` ve `f` kabul eder. Sayı olmayan girdilerde program hata ile sonlanır; farklı birim girildiğinde `error` yazar. Bu davranışlar ilk sürümün mevcut sınırlarıdır.

## Geliştirme

```sh
cargo test --locked
cargo clippy --locked -- -D warnings
cargo build --release --locked
```

GitHub Actions, her push ve pull request için testleri ve Clippy kontrolünü çalıştırır. Entegrasyon testleri programı gerçek girdilerle çalıştırır ve temel dönüşüm sonuçlarını doğrular.

Bu proje bir terminal uygulamasıdır; GitHub Pages yayını gerektirmez.

## Lisans

[MIT](LICENSE) — Copyright (c) 2026 DennisYildizdev.
