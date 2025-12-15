# 🚀 Full Stack App - SvelteKit + Rust (ntex-rs)

A high-performance full-stack application with SvelteKit frontend and Rust backend. Significantly faster than Next.js applications.

## 🏗️ Tech Stack

### Frontend
- **SvelteKit** - Web framework
- **Vite** - Development server
- **TypeScript** - Type-safe JavaScript

### Backend
- **Rust** - Systems programming language
- **ntex-rs** - Async web framework
- **ntex-cors** - CORS middleware

## 📦 Project Structure

```
.
├── front/                  # SvelteKit frontend
│   ├── src/
│   │   ├── routes/        # SvelteKit routes
│   │   ├── lib/           # Components & utilities
│   │   └── app.html       # HTML template
│   ├── build/             # Production build
│   └── vite.config.js     # Vite config
│
├── server/                 # Rust backend
│   ├── src/
│   │   ├── main.rs        # Server entry point
│   │   └── services/      # API services
│   └── Cargo.toml         # Rust dependencies
│
├── dev.sh                  # Development script
├── build.sh                # Production build script
└── README.md
```

## 🚦 Getting Started

### Prerequisites

- 📦 [Node.js](https://nodejs.org/) (v18 or higher)
- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- 📝 npm

### Installation

```bash
# Clone the repository
git clone https://github.com/crestbala/firekit.git
cd firekit
```

## 🔧 Development

```bash
./dev.sh
```

This starts both frontend (port 5173) and backend (port 8080) servers.

**Access:** `http://localhost:5173`

## 🏭 Production Build

```bash
./build.sh
```

This builds the frontend and compiles the Rust backend into a single binary.

### Run Production Server

```bash
cd server
./target/release/server
```

**Access:** `http://localhost:8080`

## 🌐 How It Works

### Development Mode
- Frontend runs on `localhost:5173` with hot reload
- Backend runs on `localhost:8080` for API
- Vite proxies `/api/*` requests to backend

### Production Mode
- Single server on `localhost:8080`
- Rust serves static files and API
- SPA fallback serves `index.html` for all routes

## 🛣️ API Routes

All API endpoints are prefixed with `/api`:

```
GET  /api/search          # Search functionality
GET  /api/random          # Get random items
```

## ⚡ Performance

This stack is significantly faster than Next.js:

- 🦀 **Rust backend** - Near-native performance
- ⚡ **ntex-rs** - One of the fastest web frameworks
- 🎯 **SvelteKit** - Smaller bundle sizes than React
- 📦 **Single binary** - Fast deployment and startup
- 🚀 **Async by default** - Efficient resource usage

## 🔑 Important Configuration

### Frontend

**Add to `front/src/app.html`:**
- Include `<base href="/" />` in the `<head>` section for proper asset loading on all routes

**Vite proxy in `front/vite.config.js`:**
- Proxy `/api` requests to `http://localhost:8080` for development

### Backend

**CORS configuration:**
- Allow `http://localhost:5173` origin in development
- Serves static files from `../front/build` in production

## 🎯 Features

- ⚡ Fast development with hot reload
- 🔒 Type-safe frontend and backend
- 🚀 High-performance Rust backend
- 🎨 Modern UI with SvelteKit
- 📦 Single binary deployment
- 🌐 RESTful API
- 🔄 Async operations

## 🐛 Troubleshooting

### Assets Not Loading on Deep Routes

Add `<base href="/" />` to `front/src/app.html` in the `<head>` section.

### API Requests Failing in Dev

Make sure both frontend and backend servers are running. Check that Vite proxy is configured correctly.

### CORS Errors

Verify CORS configuration in `server/src/main.rs` includes the frontend origin.

## 📚 Learn More

- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [ntex-rs Documentation](https://ntex.rs/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 📄 License

MIT

It is improved by me with help of Hugo Duprez's awesome work: [Hugo Duprez](https://www.hugoduprez.com/)

---

Built with SvelteKit and Rust
