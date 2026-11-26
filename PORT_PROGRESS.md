# Rust Port & Rewrite Progress Ledger

> **Hedef:** Five Nights at Freddy's 1 (FNAF 1)  
> **Orijinal Dil/Motor:** Clickteam Fusion 2.5 (.mfa + 605 PNG + 40 WAV)  
> **Hedef Rust Stack:** Macroquad + Serde + Thiserror + Rand  
> **Genel Durum:** 🟢 Kodlama & Clippy Tamamlandı  

---

## 2. Özellik & Alt Sistem Matrisi (Feature Matrix)

| No | Alt Sistem | Modül / Dosya | Kontrat & Beklenen Davranış | Test / Doğrulama Durumu | Kapı Durumu |
|:---|:---|:---|:---|:---|:---:|
| 1.1 | Core Data Models & State | `src/models.rs` | GameState, CameraId, AnimatronicId, NightConfig | `2 passed; 0 failed` | [x] Geçti |
| 2.1 | Animatronic AI & Timer | `src/ai.rs` | 4 Animatronik hareket olasılıkları, 12AM-6AM zamanlayıcı | `3 passed; 0 failed` | [x] Geçti |
| 3.1 | Office, Power & Cameras | `src/office.rs` | Kapılar, ışıklar, güç tüketimi, kesinti (outage), kamera | `2 passed; 0 failed` | [x] Geçti |
| 4.1 | Audio & Asset Manager | `src/assets.rs` | Dinamik doku/ses yükleme ve önbellekleme | `cargo clippy` geçti | [x] Geçti |
| 5.1 | Macroquad Renderer & Loop| `src/main.rs` | Ofis pan, kamera arayüzü, jumpscare, 6AM, menü | `cargo clippy --all-targets -- -D warnings` 0 uyarı | [x] Geçti |
