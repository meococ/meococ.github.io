# meococ.github.io

Portfolio cá nhân — Meo Coc, BIM Coordinator tại TP.HCM.
Editorial peak edition, thuần HTML/CSS/JS, không framework, deploy trên GitHub Pages.

## Cấu trúc

```
/
├── index.html                 Trang chủ (hero, dự án, giới thiệu, hồ sơ, ghi chép, liên hệ)
├── assets/
│   ├── style.css              Toàn bộ style dùng chung mọi trang
│   └── site.js                Intro, reveal, counter, nav, progress, copy email
├── du-an/                     Trang case study cho từng dự án
│   ├── nut-giao-cao-toc.html
│   ├── phoi-hop-depot-metro.html
│   └── tu-dong-hoa-thoat-nuoc.html
└── ghi-chep/                  Bài viết
    ├── checklist-kiem-tra-xung-dot.html
    └── dynamo-dung-mot-lan-cap-nhat-moi-noi.html
```

## Cách thay ảnh / video trong trang dự án

Trong mỗi file `du-an/*.html`, tìm các khối `.media-slot`:

```html
<div class="media-slot">
  <svg>...</svg>                        <!-- XÓA svg này -->
  <span class="cap-in">ẢNH 01 — ...</span>
</div>
```

Thay bằng ảnh/video thật (giữ lại 4 `<span class="corner">` cho đẹp):

```html
<div class="media-slot">
  <img src="/media/nut-giao-render.jpg" alt="Mô hình liên kết nút giao">
  <span class="corner c1"></span><span class="corner c2"></span>
  <span class="corner c3"></span><span class="corner c4"></span>
</div>
```

Video thì thay svg bằng `<video src="/media/demo.mp4" controls poster="/media/demo-poster.jpg"></video>`.
Ảnh/video để trong thư mục `/media/` (tạo nếu chưa có).

## Cách thêm bài viết mới

1. Copy một file trong `ghi-chep/` làm khung, đổi tên file.
2. Sửa `<title>`, `meta description`, tiêu đề `h1`, ngày ở `.post-meta` và nội dung trong `.case-grid`.
3. Thêm một hàng `.wt-row` mới trong mục Ghi chép của `index.html` (copy hàng cũ, sửa ngày/tiêu đề/link).

## Ghi chú kỹ thuật

- View Transitions API: chuyển trang mượt trên Chrome/Edge; trình duyệt khác tự dùng chuyển trang thường.
- `prefers-reduced-motion`: mọi animation được tắt đúng chuẩn.
- Chế độ preview chụp ảnh: thêm `?static=1` vào URL để tắt animation.
