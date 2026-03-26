# SOLFUNMEME DAO — Free Tier Deployments

All deployments serve the same 71-shard Gandalf WASM app (5.6MB split into ~78KB shards).

## ✅ Active Deployments (6)

| Platform | URL | Free Tier |
|----------|-----|-----------|
| GitHub Pages | https://meta-introspector.github.io/solfunmeme-dioxus/ | Always free |
| Cloudflare Pages | https://solfunmeme-dioxus.pages.dev | Always free, global CDN |
| Vercel | https://solfunmeme-dioxus.vercel.app | Always free, edge network |
| HuggingFace Spaces | https://introspector-solfunmeme-dioxus.hf.space/ | Always free (Docker SDK) |
| Oracle Cloud | https://objectstorage.us-ashburn-1.oraclecloud.com/n/id1iqr236pdp/b/solfunmeme-dioxus/o/index.html | 10GB always free |
| Self-hosted | https://solana.solfunmeme.com/dioxus/ | Own server, nginx proxy |

## 🎯 TODO: Connect via Dashboard

These platforms support GitHub integration — connect `meta-introspector/solfunmeme-dioxus`, set publish directory to `docs/`, leave build command empty:

| Platform | Setup URL | Free Tier |
|----------|-----------|-----------|
| Render | https://dashboard.render.com/select-repo → Static Site → `docs/` | Always free, 100GB/mo |
| Netlify | https://app.netlify.com/start → GitHub → `docs/` | Always free, 100GB/mo |
| Zeabur | https://zeabur.com → GitHub → `docs/` | Always free, 10GB/mo |
| Azure Static Web Apps | https://portal.azure.com → Static Web Apps → GitHub → `docs/` | Always free, 100GB/mo |

## 🔧 CLI Deploy Commands

```bash
# Build shards (requires nix)
/mnt/data1/meta-introspector/submodules/solfunmeme-dioxus/build.sh

# Cloudflare Pages
CLOUDFLARE_API_TOKEN=$(cat ~/.cloudflare-pages) \
CLOUDFLARE_ACCOUNT_ID=0ceffbadd0a04623896f5317a1e40d94 \
npx wrangler pages deploy docs/ --project-name=solfunmeme-dioxus

# Oracle OCI (Python)
python3 -c "
import oci, os, mimetypes
config = oci.config.from_file('~/.solfunmeme-keys/oci_config')
c = oci.object_storage.ObjectStorageClient(config)
ns = c.get_namespace().data
for root, _, files in os.walk('docs'):
    for f in files:
        p = os.path.join(root, f)
        ct = 'application/wasm' if f.endswith('.wasm') else mimetypes.guess_type(p)[0] or 'application/octet-stream'
        c.put_object(ns, 'solfunmeme-dioxus', os.path.relpath(p, 'docs'), open(p, 'rb'), content_type=ct)
"

# Vercel + GitHub Pages — auto-deploy on git push to main
git push origin main
```

## Submodules

- `source/solfunmeme-dioxus` — WASM frontend (Dioxus 0.7.3, 12 plugins)
- `source/erdfa-publish` — shared Rust lib + CLI + service
- `source/solfunmeme-introspector` — Lean4 verified governance proofs
- `source/pick-up-nix` — Nix development environment
- `source/wrangler-action` — Cloudflare Pages GitHub Action
