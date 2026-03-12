# ⚡ VibeStock — Rust-Powered Inventory Management System

> **MCA Final Year Project** — Full-stack real-time inventory system built with Rust, Tauri, and Kotlin.

[![Rust](https://img.shields.io/badge/Rust-1.78+-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.7-blue?logo=rust)](https://github.com/tokio-rs/axum)
[![Tauri](https://img.shields.io/badge/Tauri-v2-purple?logo=tauri)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-4-ff3e00?logo=svelte)](https://svelte.dev/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15+-336791?logo=postgresql)](https://postgresql.org/)
[![Kotlin](https://img.shields.io/badge/Kotlin-Android-7f52ff?logo=kotlin)](https://kotlinlang.org/)

---

## 📐 Architecture

```
vibestock/
├── shared/          # Rust shared crate — domain models, DTOs, error types
├── api/             # Rust Axum backend — REST API + WebSockets
│   └── migrations/  # PostgreSQL migrations & seed data
├── desktop/         # Tauri v2 desktop app — Svelte glassmorphism UI
│   ├── src/         # Svelte pages & components
│   └── src-tauri/   # Rust Tauri backend (Tauri commands)
└── android/         # Kotlin mobile scanner — CameraX + ML Kit + Retrofit
```

```
┌──────────────┐   HTTP/WS    ┌─────────────────┐    SQLx     ┌──────────────┐
│  Tauri App   │◄────────────►│  Axum API        │◄───────────►│  PostgreSQL  │
│  (Svelte UI) │              │  :3000           │             │  vibestock   │
└──────────────┘              └─────────────────┘             └──────────────┘
       ▲                              ▲
       │ WebSocket (real-time)        │ HTTP REST
       │ LowStock / StockUpdated      │
       │                       ┌──────────────┐
       └───────────────────────│ Kotlin App   │
                               │ CameraX Scan │
                               └──────────────┘
```

---

## 🚀 Features

| Feature | Technology |
|---|---|
| Blazing-fast REST API | Rust + Axum + Tower |
| Real-time stock alerts | Axum WebSockets + broadcast channel |
| JWT authentication | `jsonwebtoken` + bcrypt |
| Database migrations | SQLx + PostgreSQL |
| Glassmorphism desktop UI | Tauri v2 + Svelte |
| Barcode scanning | CameraX + ML Kit (on-device) |
| Mobile stock updates | Kotlin + Retrofit → Rust API |
| Full audit log | Immutable stock movements table |

---

## 📦 Project Structure

### `shared/` — Domain Models
- `models.rs` — `Product`, `Category`, `Supplier`, `StockMovement`, `User`
- `dto.rs`    — Request/response DTOs, `PaginatedResponse<T>`, `WsEvent`
- `errors.rs` — `AppError` with `IntoResponse` impl for clean JSON errors

### `api/` — Axum Backend
- **Auth**: `POST /api/auth/login` → JWT, bcrypt password hashing
- **Products**: Full CRUD + barcode lookup + paginated search
- **Categories**: Full CRUD
- **Suppliers**: Full CRUD
- **Stock Movements**: Create (atomically updates product stock) + paginated list
- **Dashboard**: `/api/dashboard/stats` — 6 aggregated KPIs
- **WebSocket**: `/api/ws` — broadcasts `StockUpdated`, `LowStock` events

### `desktop/` — Tauri v2 + Svelte
- **Login** — animated blob background, JWT auth
- **Dashboard** — 6 KPI cards, recent movements table, live WS refresh
- **Products** — searchable paginated table, Add/Edit modal with all fields
- **Suppliers** — management table with modal
- **Stock Movements** — immutable audit log, new movement modal with type hint

### `android/` — Kotlin Scanner
- **MainActivity** — JWT login, SharedPreferences token persistence
- **ScannerActivity** — CameraX + ML Kit EAN-13/QR detection
- **ApiClient.kt** — Retrofit + Moshi models mirroring Rust DTOs

---

## ⚙️ Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | 1.78+ | [rustup.rs](https://rustup.rs/) |
| PostgreSQL | 15+ | [postgresql.org](https://postgresql.org/) |
| SQLx CLI | latest | `cargo install sqlx-cli` |
| Node.js | 20+ | [nodejs.org](https://nodejs.org/) |
| Tauri CLI | v2 | `cargo install tauri-cli` |
| Android Studio | 2024+ | [developer.android.com](https://developer.android.com/studio) |

---

## 🗄️ Database Setup

```bash
# 1. Create the database
psql -U postgres -c "CREATE DATABASE vibestock;"

# 2. Copy and edit environment file
cp .env.example .env

# 3. Run migrations + seed data
cd api
cargo sqlx migrate run

# Or use the SQLx CLI directly:
sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5432/vibestock"
```

**Default seed credentials:**

| Username | Password | Role |
|---|---|---|
| `admin` | `Password@123` | Admin |
| `manager` | `Password@123` | Manager |
| `staff1` | `Password@123` | Staff |

---

## 🦀 Running the API

```bash
cd d:/project/vibestock

# Development (with hot-reload via cargo-watch)
cargo watch -x "run -p api"

# Or directly
cargo run -p api

# API docs (manual):
# GET  http://localhost:3000/api/health
# POST http://localhost:3000/api/auth/login
# WS   ws://localhost:3000/api/ws
```

---

## 🖥️ Running the Desktop App

```bash
cd desktop

# Install frontend dependencies
npm install

# Development mode (launches Tauri window + Vite dev server)
cargo tauri dev

# Production build
cargo tauri build
```

---

## 📱 Running the Android Scanner

1. Open `android/` in Android Studio.
2. Edit `app/build.gradle.kts` → set `API_BASE_URL` to your machine's LAN IP:
   ```kotlin
   buildConfigField("String", "API_BASE_URL", "\"http://192.168.1.x:3000\"")
   ```
3. Connect your Android device (API 26+) or use the emulator.
4. Click **Run** ▶️.
5. Log in with `admin / Password@123`, point camera at any EAN-13 barcode.

---

## 🔌 API Reference

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| `POST` | `/api/auth/login` | ✗ | Get JWT token |
| `GET`  | `/api/health` | ✗ | Health check |
| `GET`  | `/api/dashboard/stats` | ✓ | 6 KPI aggregates |
| `GET`  | `/api/products` | ✓ | List (paginated, searchable) |
| `POST` | `/api/products` | ✓ | Create product |
| `GET`  | `/api/products/:id` | ✓ | Get by ID |
| `GET`  | `/api/products/barcode/:code` | ✓ | Barcode lookup (Android) |
| `PUT`  | `/api/products/:id` | ✓ | Update product |
| `DELETE` | `/api/products/:id` | ✓ | Soft-delete |
| `GET`  | `/api/categories` | ✓ | List categories |
| `POST` | `/api/categories` | ✓ | Create category |
| `GET`  | `/api/suppliers` | ✓ | List suppliers |
| `POST` | `/api/suppliers` | ✓ | Create supplier |
| `GET`  | `/api/movements` | ✓ | Paginated audit log |
| `POST` | `/api/movements` | ✓ | Record stock movement |
| `WS`   | `/api/ws` | ✗ | Real-time event stream |

---

## 🧪 WebSocket Events

```json
// Stock updated (any movement recorded)
{ "event": "StockUpdated", "payload": { "product_id": "...", "product_name": "Laptop", "new_quantity": 23 } }

// Low stock warning (quantity ≤ reorder_level)
{ "event": "LowStock", "payload": { "product_id": "...", "product_name": "Webcam", "quantity": 3, "reorder_level": 5 } }
```

---

## 📊 Database Schema

```sql
users            -- JWT auth, role: admin | manager | staff
categories       -- Product groupings
suppliers        -- Vendor master data
products         -- SKU, barcode, price, stock qty, reorder level
stock_movements  -- Immutable audit log (never deleted)
```

---

## 🏗️ Tech Stack Summary

| Layer | Technology | Purpose |
|---|---|---|
| **API** | Rust + Axum + SQLx | Type-safe, zero-cost backend |
| **DB** | PostgreSQL + migrations | ACID-compliant inventory data |
| **Auth** | JWT + bcrypt | Stateless token auth |
| **Real-time** | Tokio broadcast + WS | Live stock change notifications |
| **Desktop UI** | Tauri v2 + Svelte | 60fps native-feel Windows app |
| **Design** | Glassmorphism CSS | Dark violet/cyan premium aesthetic |
| **Mobile** | Kotlin + CameraX + ML Kit | On-device barcode scanning |
| **API Client** | Retrofit + Moshi | Kotlin ↔ Rust API typesafe bridge |

---

## 📄 License

MIT © 2024 VibeStock MCA Final Year Project
