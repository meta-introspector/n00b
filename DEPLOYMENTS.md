# SOLFUNMEME DAO — Free Tier Deployments

All deployments serve the same 71-shard Gandalf WASM app (5.6MB split into ~78KB shards).

## ✅ Active Deployments

| Platform | URL | Free Tier |
|----------|-----|-----------|
| GitHub Pages | https://meta-introspector.github.io/solfunmeme-dioxus/ | Always free |
| Cloudflare Pages | https://solfunmeme-dioxus.pages.dev | Always free, global CDN |
| Vercel | https://solfunmeme-dioxus.vercel.app | Always free, edge network |
| HuggingFace Spaces | https://introspector-solfunmeme-dioxus.hf.space/ | Always free (Docker SDK) |
| Self-hosted | https://solana.solfunmeme.com/dioxus/ | Own server, nginx proxy |

## 🎯 TODO: Additional Free Tier Platforms

From [free-resources.md](free-resources.md):

| Platform | Free Tier | Status |
|----------|-----------|--------|
| Render | Always free static sites, 100GB bandwidth | TODO |
| Netlify | Always free static sites | TODO |
| Zeabur | Always free static sites, 10GB outbound | TODO |
| AWS S3 + CloudFront | 5GB S3 + 50GB CloudFront (12 months) | TODO |
| Google Cloud Storage | 5GB-months regional (always free) | TODO |
| Azure Static Web Apps | Always free, 100GB bandwidth | TODO |
| Oracle Cloud Object Storage | 10GB always free | TODO |

## Deploy to a New Platform

The `docs/` directory in `solfunmeme-dioxus` contains the complete static build:
- 71 WASM shards + manifest.json
- shard-loader.js (parallel fetch, progress bar)
- index.html with importmap for WASM env module
- 404.html for SPA routing

Any static hosting platform works. Just point it at `docs/`.

## Build from Source

```bash
# Clone and build
git clone https://github.com/meta-introspector/solfunmeme-dioxus
cd solfunmeme-dioxus
./build.sh  # requires nix

# Or use pre-built docs/
ls docs/
```

## Submodules

This repo includes source references:
- `source/solfunmeme-dioxus` — WASM frontend (Dioxus 0.7.3, 12 plugins)
- `source/erdfa-publish` — shared Rust lib + CLI + service
- `source/solfunmeme-introspector` — Lean4 verified governance proofs
- `source/pick-up-nix` — Nix development environment
- `source/wrangler-action` — Cloudflare Pages GitHub Action
