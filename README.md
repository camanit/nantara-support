# ⚡ Nantara Support: AI-Powered IT Support Agent & Control Panel

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Web Framework](https://img.shields.io/badge/Backend-Axum-blue.svg)](https://github.com/tokio-rs/axum)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey.svg)]()
[![License](https://img.shields.io/badge/License-MIT-green.svg)]()

**Nantara Support** adalah aplikasi agen diagnostik IT Support dan Control Panel berbasis web yang ditulis dalam bahasa **Rust**. Aplikasi ini dirancang untuk mendeteksi, menguji telemetri, dan memberikan tindakan perbaikan **1-Klik (Self-Healing)** untuk masalah umum sistem operasi seperti jaringan, printer spooler, perbaikan OS, dan pembersihan memori.

![Dashboard Preview](https://via.placeholder.com/1200x600/0b0f19/06b6d4?text=Nantara+Support+Control+Panel+Dashboard)

---

## ✨ Fitur Utama (Community Edition)

* ⚡ **High Performance & Ringan**: Ditulis dengan Rust (Axum + Tokio), hanya membutuhkan memori RAM **< 15 MB**.
* 📦 **100% Portable Single-File Executable**: Seluruh berkas Web UI tertanam langsung di dalam `nantara-support.exe`.
* 🌐 **Web Control Panel Modern**: Dashboard visual berdesain *Dark Mode & Glassmorphism* dengan grafik telemetri real-time.
* 🛠️ **1-Click Repair Actions**:
  * **Fix Network & DNS**: Otomatis `flushdns`, `release/renew IP`, reset adapter.
  * **Fix Printer Spooler**: Membersihkan antrean dokumen mogok & restart service `spooler`.
  * **System Integrity Repair**: Menjalankan perbaikan file sistem Windows (`sfc /scannow` & `DISM`).
  * **Active Directory GPUpdate**: Memaksa pembaruan kebijakan group policy (`gpupdate /force`).
* 🔥 **Top Heavy Process Manager**: Menampilkan aplikasi boros CPU/RAM dan opsi `taskkill` dalam 1 klik.
* 🖥️ **Real-Time Execution Logs**: Menampilkan output console STDOUT & STDERR dari perintah Windows secara transparan.

---

## ⚡ Fitur Pro / Enterprise Edition (Lisensi)

| Fitur | Community (Free) | Pro / Enterprise 🔑 |
| :--- | :---: | :---: |
| **Core OS Diagnostics & 1-Click Repair** | ✅ | ✅ |
| **Local Web Control Panel** | ✅ | ✅ |
| **Real-time System Telemetry** | ✅ | ✅ |
| **🧠 AI Helpdesk Natural Language Chatbot** | ❌ | ✅ |
| **🔍 AI BSOD & Event Viewer Crash Log Analyzer** | ❌ | ✅ |
| **🌐 Central Multi-Machine RMM Management** | ❌ | ✅ |
| **🔑 Active Directory & Password Self-Service** | ❌ | ✅ |

---

## 🚀 Panduan Memulai (Quick Start)

### Prasyarat
* [Rust](https://www.rust-lang.org/tools/install) (cargo v1.75+)
* Windows 10/11 (dengan hak akses Administrator untuk perintah perbaikan sistem)

### 1. Clone Repository
```bash
git clone https://github.com/camanit/nantara-support.git
cd nantara-support
```

### 2. Jalankan Agent & Web Server
```bash
cargo run
```

### 3. Akses Control Panel
Buka browser favorit Anda dan kunjungi:
```text
http://localhost:3030
```

---

## 🏗️ Struktur Proyek

```text
nantara-support/
├── Cargo.toml                  # Konfigurasi Package & Dependensi Rust
├── README.md                   # Dokumentasi Utama
├── ROADMAP.md                  # Roadmap Pengembangan Proyek
├── license.key                 # File Lisensi Pro (Opsional)
├── src/
│   ├── main.rs                 # Inisialisasi Server Axum & Portable Assets
│   ├── ai/                     # Engine Gemini AI & Smart Fallback
│   ├── license/                # Verifikator Lisensi Kriptografis
│   ├── api/
│   │   └── routes.rs           # REST API Endpoints
│   ├── diagnostics/            # Engine Perbaikan Sistem & Log Parser
│   └── utils/
│       └── cmd.rs              # Executor PowerShell & CMD
├── tools/
│   └── key-generator/          # Tool Privat Generator Lisensi Pro
└── web/                        # Web Dashboard UI (Di-embed ke .exe)
```

---

## 💖 Dukungan & Donasi (Support & Donation)

Jika proyek open-source **Nantara Support** ini bermanfaat bagi Anda atau tim IT Anda, Anda dapat memberikan dukungan dan donasi untuk kelanjutan pengembangan proyek melalui:

* 🏦 **Bank**: Allo Bank
* 💳 **No. Rekening**: `081260006666`
* 💬 **Konfirmasi / WhatsApp**: [+62 812-6000-6666](https://wa.me/6281260006666)

Dukungan Anda sangat berarti untuk kelangsungan dan pembaruan fitur Nantara Support ke depan!

---

## 🛡️ Lisensi

Proyek open-source ini dilisensikan di bawah [MIT License](LICENSE).  
Fitur Pro & Enterprise dilindungi oleh Lisensi Kriptografis Digital terenkripsi.
