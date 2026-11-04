# meococ.github.io

Portfolio cá nhân — Huỳnh Trung (Meo Coc), BIM Coordinator tại TP.HCM.
Editorial/brutalist design, render bởi **`meogen`** — static site generator viết tay bằng Rust, deploy trên GitHub Pages.

## meogen

Một binary nhỏ (~400 dòng), không framework:

```sh
cargo run -- build          # render content/ → dist/
cargo run -- serve          # dev server, rebuild mỗi request
cargo run -- serve --port 9000
```

- `content/` — một file = một trang. HTML fragment hoặc Markdown (ghi chép), front matter TOML giữa cặp `+++`.
- `templates/` — Tera. `base.html` là chrome chung (header, mobile menu, lang toggle, footer); `post.html` extend nó cho bài viết.
- `static/` — copy nguyên vào `dist/` (css, js, favicon, CV pdf).
- `dist/` — output, deploy lên Pages qua GitHub Actions.

Front matter tối thiểu: `title`, `description`. Tuỳ chọn: `lang` (`vi`/`en`, mặc định suy từ thư mục `en/`), `section` (nav active), `intro`, `marquee`, `rail`, `read_bar`, `kind = "post"` (markdown → template post), `h1`, `kicker`, `date`, `minutes`, `tag`, `back_href`, `back_label`, `foot_tag`, `nav` (prev/next).

## Cấu trúc

```
/
├── src/main.rs              meogen: build + serve
├── content/                 VI: trang chủ, du-an/, ghi-chep/ · EN: en/
├── templates/               base.html, post.html
├── static/assets/           style.css, site.js, CV pdf
└── .github/workflows/       Pages deploy
```
