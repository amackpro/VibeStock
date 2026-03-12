# 📦 VibeStock — Modern Inventory Management System

**VibeStock** is a fully-featured, cross-platform inventory management system built as an MCA Final Year Project. It features a high-performance Rust backend, a beautiful glassmorphism desktop application, and a Kotlin Android barcode scanner.

![VibeStock Overview](https://img.shields.io/badge/Status-Complete-success?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=Tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)
![Kotlin](https://img.shields.io/badge/Kotlin-0095D5?style=for-the-badge&logo=kotlin&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

---

## 🏗️ Architecture & Tech Stack

VibeStock uses a modern, high-performance tech stack divided into three main clients connected to a central API.

### 1. Backend API (`api/` and `shared/`)
* **Language**: Rust
* **Framework**: Axum (high-performance async web framework)
* **Database**: PostgreSQL (with standard hex-UUIDs)
* **ORM / Queries**: SQLx (async, purely Rust SQL toolkit)
* **Real-time**: WebSockets (Axum Broadcast channels)
* **Security**: JWT Authentication + Bcrypt password hashing

### 2. Desktop Application (`desktop/`)
* **Framework**: Tauri v2 (Rust-backed lightweight desktop apps)
* **Frontend UI**: Svelte + Vite
* **Design System**: Custom CSS Glassmorphism (Vibrant gradients, frosted glass, smooth animations)
* **Features**: Live dashboard with 6 KPIs, real-time WebSocket stock alerts, CRUD modules for products and suppliers.

### 3. Mobile Scanner (`android/`)
* **Language**: Kotlin (Android Native)
* **Scanning SDK**: Google ML Kit + AndroidX CameraX (fast, on-device barcode parsing)
* **Networking**: Retrofit 2 + Moshi (API client)
* **Features**: Scan barcodes instantly, query stock, submit stock movements (IN/OUT/ADJUSTMENT).

---

## ✨ Core Features

* **🔐 Role-Based Access**: Secure JWT logins for Admins, Managers, and Staff.
* **📦 Product Management**: Full CRUD for products, categories, and suppliers.
* **🔄 Immutable Stock Log**: All stock changes (IN, OUT, ADJUSTMENT) are recorded as immutable movements for perfect auditing.
* **⚡ Real-time Alerts**: When a stock movement occurs, WebSockets instantly broadcast the new stock levels and Low Stock alerts to all connected desktop clients without a page refresh.
* **📱 Mobile Integration**: Factory staff can use Android phones to scan barcodes and update inventory instantly over the local network.

---

## 🚀 Setup & Installation

### Prerequisites
1. **Rust**: Installed via `rustup` (v1.75+)
2. **Node.js**: Installed (v18+)
3. **PostgreSQL**: Installed (v15+) and running
4. **Android Studio**: (Optional) For the mobile scanner app.

---

### Step 1: Database Setup
1. Open PostgreSQL (`psql` or pgAdmin).
2. Create an empty database:
   ```sql
   CREATE DATABASE vibestock;
   ```
3. Copy the environment config template:
   ```bash
   cp .env.example .env
   ```
4. Open `.env` and configure your `DATABASE_URL` with your PG credentials.
   *(Example: `postgres://postgres:password@localhost:5432/vibestock`)*

### Step 2: Running the API Server
The API manages the database migrations automatically on startup.
```bash
# From the project root directory
cargo run -p api
```
*(You will see "✅ Migrations applied" and the server will start on `http://localhost:3000`)*

**Demo Accounts (Password for all is `Password@123`):**
* `admin`
* `manager`
* `staff1`

### Step 3: Running the Desktop Application
The desktop app requires the API to be running first.
```bash
# Open a new terminal
cd desktop

# Install frontend dependencies
npm install

# Start the Tauri application
cargo tauri dev
# or alternatively: npx @tauri-apps/cli dev
```

### Step 4: Running the Android Scanner
1. Open the `android/` folder in **Android Studio**.
2. Wait for Gradle to sync dependencies.
3. Open `android/app/build.gradle.kts`.
4. Look for the `API_BASE_URL` config variable:
   - If using the **Android Emulator**, keep it as `"http://10.0.2.2:3000"`.
   - If using a **Physical Android Phone**, change it to your computer's local Wi-Fi IP address (e.g., `"http://192.168.1.55:3000"`).
5. Press **Run** (▶️) to install the app on your device.

---

## 📁 Project Structure

```text
vibestock/
├── Cargo.toml                 # Workspace root config
├── .env                       # Environment variables (DB, JWT secret)
│
├── shared/                    # [Rust] Shared structs and DTOs
│   ├── src/models.rs          # DB models (Product, StockMovement, etc)
│   └── src/errors.rs          # Unified Application Errors
│
├── api/                       # [Rust] Axum Backend
│   ├── migrations/            # SQLx database schema & seed data
│   └── src/                   # Route handlers, auth, ws, and db setup
│
├── desktop/                   # [Rust+Svelte] Tauri App
│   ├── src-tauri/             # Tauri backend / capabilities / icons
│   └── src/                   # Svelte UI, Routes, Glassmorphism CSS
│
└── android/                   # [Kotlin] Mobile Scanner App
    └── app/src/main/java/     # CameraX, ML Kit, Retrofit logic
```

---
*Created for MCA Final Year Project Submission — 2024*
