//! meogen — the tiny static site generator behind meococ.github.io.
//!
//! `meogen build` renders every file in `content/` through the Tera templates
//! in `templates/` and copies `static/` verbatim, writing the site to `dist/`.
//! `meogen serve` serves `dist/` over HTTP and rebuilds on every request, so
//! the browser always sees the latest files — no watch mode needed.

use std::env;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

use serde::Serialize;
use tera::{Context, Tera};
use walkdir::WalkDir;

const CONTENT: &str = "content";
const STATIC: &str = "static";
const TEMPLATES: &str = "templates/*.html";
const DIST: &str = "dist";

/// Page metadata parsed from the `+++` TOML block on top of a content file.
/// Everything optional falls back to sane defaults in `page_context`.
#[derive(Debug, Default, serde::Deserialize)]
struct Front {
    title: String,
    /// Post h1 when it differs from `<title>` (which carries the " — name" suffix).
    h1: Option<String>,
    description: String,
    lang: Option<String>,
    /// "page" (default, body is raw HTML) or "post" (body is Markdown).
    kind: Option<String>,
    /// Nav key that receives the `.act` class ("projects" | "writing" | "").
    section: Option<String>,
    /// Left-rail text; empty string hides the rail.
    rail: Option<String>,
    /// Splash intro (home only).
    intro: Option<bool>,
    /// Footer marquee (home only).
    marquee: Option<bool>,
    /// Middle cell of the footer meta row.
    foot_tag: Option<String>,
    /// Reading-progress bar (posts).
    read_bar: Option<bool>,
    /// Post hero fields.
    kicker: Option<String>,
    date: Option<String>,
    minutes: Option<String>,
    tag: Option<String>,
    back_href: Option<String>,
    back_label: Option<String>,
    /// `case-nav` links at the bottom of a post.
    nav: Option<Vec<NavItem>>,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
struct NavItem {
    label: String,
    href: String,
    title: String,
}

#[derive(Serialize)]
struct NavLink {
    key: String,
    label: String,
    href: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1).as_deref() {
        None | Some("build") => build(),
        Some("serve") => {
            let port: u16 = env::args()
                .nth(2)
                .and_then(|p| p.parse().ok())
                .unwrap_or(8000);
            serve(port)
        }
        Some(other) => {
            eprintln!("meogen: lệnh lạ '{other}' — dùng `build` hoặc `serve [port]`");
            std::process::exit(2);
        }
    }
}

fn build() -> Result<(), Box<dyn Error>> {
    let dist = Path::new(DIST);
    if dist.exists() {
        fs::remove_dir_all(dist)?;
    }
    fs::create_dir_all(dist)?;

    for entry in WalkDir::new(STATIC).min_depth(1) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(STATIC)?;
        let out = dist.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&out)?;
        } else {
            fs::create_dir_all(out.parent().expect("file path has a parent"))?;
            fs::copy(entry.path(), &out)?;
        }
    }

    let tera = Tera::new(TEMPLATES)?;
    let mut pages = 0;
    for entry in WalkDir::new(CONTENT).min_depth(1) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let ext = entry
            .path()
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        if ext != "html" && ext != "md" {
            continue;
        }

        let rel = entry.path().strip_prefix(CONTENT)?.with_extension("html");
        let raw = fs::read_to_string(entry.path())?;
        let (front, body_raw) = split_front(&raw, entry.path())?;
        let mut ctx = page_context(&front, &rel);
        let template = if ext == "md" || front.kind.as_deref() == Some("post") {
            ctx.insert("body", &markdown_to_html(&body_raw));
            "post.html"
        } else {
            ctx.insert("body", &body_raw);
            "base.html"
        };
        let rendered = tera.render(template, &ctx)?;

        let out = dist.join(&rel);
        fs::create_dir_all(out.parent().expect("file path has a parent"))?;
        fs::write(&out, rendered)?;
        pages += 1;
    }
    println!("meogen: {pages} trang → {DIST}/");
    Ok(())
}

/// Splits a `+++`-delimited front-matter block from the body.
fn split_front(raw: &str, path: &Path) -> Result<(Front, String), Box<dyn Error>> {
    let raw = raw.replace("\r\n", "\n");
    let trimmed = raw.trim_start();
    if let Some(rest) = trimmed.strip_prefix("+++") {
        if let Some(end) = rest.find("\n+++") {
            let front: Front = toml::from_str(&rest[..end])
                .map_err(|e| format!("{}: front matter: {e}", path.display()))?;
            return Ok((front, rest[end + 4..].trim_start().to_string()));
        }
    }
    Ok((Front::default(), raw))
}

/// Everything the base template needs besides the page body.
/// `lang` defaults from the `en/` directory convention when front matter doesn't say.
fn page_context(front: &Front, rel: &Path) -> Context {
    let lang = front.lang.clone().unwrap_or_else(|| {
        if rel.starts_with("en") {
            "en".into()
        } else {
            "vi".into()
        }
    });
    let en = lang == "en";
    let home = if en { "/en/" } else { "/" };
    let section = front.section.clone().unwrap_or_default();

    let mut ctx = Context::new();
    ctx.insert("title", &front.title);
    ctx.insert("description", &front.description);
    ctx.insert("lang", &lang);
    ctx.insert("home", home);
    ctx.insert("section", &section);
    ctx.insert("rail", front.rail.as_deref().unwrap_or_default());
    ctx.insert("intro", &front.intro.unwrap_or(false));
    ctx.insert("marquee", &front.marquee.unwrap_or(false));
    ctx.insert("read_bar", &front.read_bar.unwrap_or(false));
    ctx.insert("foot_tag", front.foot_tag.as_deref().unwrap_or_default());

    ctx.insert("title_h1", front.h1.as_deref().unwrap_or(&front.title));
    ctx.insert("kicker", front.kicker.as_deref().unwrap_or_default());
    ctx.insert("date", front.date.as_deref().unwrap_or_default());
    ctx.insert("minutes", front.minutes.as_deref().unwrap_or_default());
    ctx.insert("tag", front.tag.as_deref().unwrap_or_default());
    ctx.insert("back_href", front.back_href.as_deref().unwrap_or_default());
    ctx.insert(
        "back_label",
        front.back_label.as_deref().unwrap_or_default(),
    );
    ctx.insert("nav", &front.nav.clone().unwrap_or_default());

    ctx.insert("nav_items", &nav_items(en, home));
    ctx.insert("lang_switch", if en { "/" } else { "/en/" });
    ctx.insert("other_lang", if en { "vi" } else { "en" });
    ctx.insert("avail", if en { "OPEN 2026" } else { "SẴN SÀNG 2026" });
    ctx.insert(
        "foot_left",
        if en {
            "© 2026 HUYNH TRUNG — MADE IN SAIGON · MEOCOC'S SITE"
        } else {
            "© 2026 HUỲNH TRUNG — LÀM TẠI SÀI GÒN · SITE CỦA MÈO CỌC"
        },
    );
    ctx.insert(
        "foot_top",
        if en {
            "BACK TO TOP ↑"
        } else {
            "LÊN ĐẦU TRANG ↑"
        },
    );
    ctx.insert(
        "skip",
        if en {
            "Skip to main content"
        } else {
            "Tới nội dung chính"
        },
    );
    ctx.insert(
        "nav_aria",
        if en {
            "Main navigation"
        } else {
            "Điều hướng chính"
        },
    );
    ctx.insert("menu_label", if en { "Open menu" } else { "Mở menu" });
    ctx.insert(
        "intro_sub",
        if en {
            "HUYNH TRUNG — BIM COORDINATION · MEOCOC'S SITE"
        } else {
            "HUỲNH TRUNG — ĐIỀU PHỐI BIM · SITE CỦA MÈO CỌC"
        },
    );
    ctx.insert("marquee_items", &marquee_items(en));
    ctx
}

fn nav_items(en: bool, home: &str) -> Vec<NavLink> {
    let labels: [(&str, &str, &str); 5] = if en {
        [
            ("projects", "PROJECTS", "#projects"),
            ("about", "ABOUT", "#about"),
            ("cv", "RÉSUMÉ", "#cv"),
            ("writing", "NOTES", "#writing"),
            ("contact", "CONTACT", "#contact"),
        ]
    } else {
        [
            ("projects", "DỰ ÁN", "#projects"),
            ("about", "GIỚI THIỆU", "#about"),
            ("cv", "HỒ SƠ", "#cv"),
            ("writing", "GHI CHÉP", "#writing"),
            ("contact", "LIÊN HỆ", "#contact"),
        ]
    };
    labels
        .into_iter()
        .map(|(key, label, anchor)| NavLink {
            key: key.into(),
            label: label.into(),
            href: format!("{home}{anchor}"),
        })
        .collect()
}

fn marquee_items(en: bool) -> Vec<String> {
    if en {
        ["HUYNH TRUNG", "BIM COORDINATION", "SYSTEMS BUILDER"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        ["HUỲNH TRUNG", "ĐIỀU PHỐI BIM", "NGƯỜI XÂY HỆ THỐNG"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}

/// Renders a Markdown post body, tagging block elements with `.rv` so they
/// share the scroll-reveal animation used across the site.
fn markdown_to_html(md: &str) -> String {
    let parser = pulldown_cmark::Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html.replace("<p>", "<p class=\"rv\">")
        .replace("<h2>", "<h2 class=\"rv\">")
        .replace("<ul>", "<ul class=\"rv\">")
        .replace("<blockquote>", "<blockquote class=\"rv\">")
        .replace("<pre>", "<pre class=\"rv\">")
}

/// Serves `dist/`, rebuilding the site before every request.
fn serve(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("meogen serve — http://127.0.0.1:{port} (rebuild mỗi request)");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle(stream),
            Err(e) => eprintln!("meogen: kết nối lỗi: {e}"),
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream) {
    let request_line = match BufReader::new(&mut stream).lines().next() {
        Some(Ok(line)) => line,
        _ => return,
    };
    let mut path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string();
    if path.ends_with('/') {
        path.push_str("index.html");
    }
    // chặn path traversal: bỏ mọi thành phần ".."
    let rel: PathBuf = Path::new(&path)
        .components()
        .filter_map(|c| match c {
            std::path::Component::Normal(p) => Some(p),
            _ => None,
        })
        .collect();
    let file = Path::new(DIST).join(&rel);

    if let Err(e) = build() {
        eprintln!("meogen: build lỗi: {e}");
    }
    let (status, body, ctype) = match fs::read(&file) {
        Ok(bytes) => ("200 OK", bytes, content_type(&file)),
        Err(_) => (
            "404 Not Found",
            b"meogen 404".to_vec(),
            "text/plain; charset=utf-8",
        ),
    };
    let header = format!(
        "HTTP/1.1 {status}\r\ncontent-type: {ctype}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(&body);
    let _ = stream.flush();
}

fn content_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
    {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "pdf" => "application/pdf",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "txt" | "md" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_matter_parses() {
        let raw = "+++\ntitle = \"Hi\"\ndescription = \"d\"\nlang = \"en\"\n+++\n<body>x</body>";
        let (front, body) = split_front(raw, Path::new("t")).unwrap();
        assert_eq!(front.title, "Hi");
        assert_eq!(front.lang.as_deref(), Some("en"));
        assert_eq!(body, "<body>x</body>");
    }

    #[test]
    fn markdown_posts_get_reveal_class() {
        let html = markdown_to_html("# T\n\nhello **world**");
        assert!(html.contains("<p class=\"rv\">"));
    }

    #[test]
    fn traversal_is_stripped() {
        let path = "/../Cargo.toml";
        let rel: PathBuf = Path::new(path)
            .components()
            .filter_map(|c| match c {
                std::path::Component::Normal(p) => Some(p),
                _ => None,
            })
            .collect();
        assert_eq!(rel, PathBuf::from("Cargo.toml"));
    }
}
