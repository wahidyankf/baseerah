# Brand Context Reference

Per-app brand guidance for UI development decisions.

## baseerah-fe

- **Product**: Baseerah — a personal operating layer (AI assistant, content builder, posting
  helper, personal workflow engine)
- **URL**: TBD (walking-skeleton phase; see [Baseerah Vision](../../../../repo-governance/vision/beaver-nest.md))
- **Audience**: One maintainer (single-tenant, personal tool — not multi-tenant SaaS)
- **Personality**: Clear, insightful, self-owned — بصيرة (insight, inner vision)
- **Tone**: Direct, personal, no marketing gloss
- **Palette**: Indigo-violet OKLCH — `--color-primary: var(--hue-sky)` (hue 265, insight/clarity);
  see `libs/web-ui-token/src/baseerah.css`
- **Typography**: Shared `web-ui-token` typography scale; font loading via next/font
- **Unique tokens**: none yet — the hello-world page uses only the shared semantic tokens
- **UI character**: Minimal — current scope is a single landing page (header/main/footer shell)
  rendering a greeting fetched from `baseerah-be`
- **Framework**: Next.js 16, Tailwind v4, shadcn/ui

## ayokoding-web

- **Product**: Educational coding platform (AyoKoding)
- **URL**: ayokoding.com
- **Audience**: Indonesian tech community, developers learning programming
- **Personality**: Approachable, educational, encouraging
- **Tone**: Informal, tutorial-oriented, bilingual (English + Indonesian)
- **Palette**: Blue-tinted — `--primary: hsl(221.2 83.2% 53.3%)` (vibrant blue)
- **Typography**: System font with font-feature-settings for ligatures
- **Unique tokens**: `--sidebar-*` (8 tokens) for navigation sidebar
- **UI character**: Content-focused, long-form reading, code blocks with syntax highlighting
- **Framework**: Next.js 16, Tailwind v4 + @tailwindcss/typography, shadcn/ui, rehype-pretty-code
