+++
title = "Dynamo: dựng một lần, cập nhật mọi nơi — Huỳnh Trung"
h1 = "Dynamo: dựng một lần, cập nhật mọi nơi"
description = "Script nhỏ, tiết kiệm hàng chục giờ lặp lại. Cách mình dùng Dynamo + Python để bảng tính điều khiển mô hình thay vì ngược lại."
kind = "post"
section = "writing"
read_bar = true
rail = "HUỲNH TRUNG — GHI CHÉP"
foot_tag = "GHI CHÉP · TỰ ĐỘNG HÓA"
kicker = "GHI CHÉP · TỰ ĐỘNG HÓA"
date = "2026.10"
minutes = "5 PHÚT ĐỌC"
tag = "TỰ ĐỘNG HÓA"
back_href = "/#writing"
back_label = "TẤT CẢ GHI CHÉP"
nav = [
  { label = "← BÀI TRƯỚC", href = "/ghi-chep/checklist-kiem-tra-xung-dot.html", title = "Checklist kiểm tra xung đột cho buổi họp phối hợp" },
  { label = "DỰ ÁN LIÊN QUAN →", href = "/du-an/tu-dong-hoa-thoat-nuoc.html", title = "Tự động hóa mạng thoát nước" },
]
+++

Có một loại công việc trong nghề xây dựng mà ai cũng chấp nhận như luật trời: **việc lặp lại**. Hố ga thứ 37 giống hố ga thứ 36, chỉ khác cao độ đáy. Bản vẽ lòng đường sửa, thì 200 nhãn trên 20 sheet phải sửa theo. Mình không chấp nhận luật trời đó.

## Bảng tính là nguồn sự thật

Nguyên tắc mình chọn rất đơn giản: **mô hình không phải nơi lưu dữ liệu — bảng tính mới là**. Kỹ thuật đã quen làm việc với Excel, vậy thì cứ để bảng tính làm nguồn sự thật, còn script chỉ có một nhiệm vụ: đọc và dựng.

- Mỗi hố ga một dòng: tọa độ X/Y, cao độ miệng, cao độ đáy, kích thước.
- Mỗi đoạn ống một dòng: hố ga đầu, hố ga cuối, đường kính, độ dốc.
- Script đọc toàn bộ, kiểm tra hợp lệ, rồi đặt hình vào mô hình — kèm báo cáo lỗi nếu dữ liệu thiếu.

Luồng xử lý của script giữ thật ngắn, để ai cũng đọc hiểu được:

```
doc = mở tài khoản Civil 3D
rows = đọc("mang-luoi-thoat-nuoc.xlsx")
for row in rows:
    kiểm_tra(row)          # thiếu cao độ? dừng, báo dòng lỗi
    đặt_hố_ga(row)         # family + tham số từ bảng tính
    đặt_ống(row.a, row.b)  # nối theo cặp, cao độ đáy theo độ dốc
ghi_log("đã đặt", len(rows))
cập_nhật_tất_cả_nhãn()
```

## Ba bài học rút ra

- **Script phải chết một cách êm ái.** Dữ liệu sai thì dừng lại và nói rõ lỗi ở dòng nào — đừng bao giờ đặt hình sai rồi để người khác phát hiện ngoài công trường.
- **Tài liệu hóa là phần của script.** Một script chỉ mình chạy được thì không phải tự động hóa — đó chỉ là cách mình làm việc nhanh hơn một người.
- **Đừng tự động hóa thứ chưa ổn định.** Quy tắc đặt tên còn đang đổi mỗi tuần? Ổn định nó trước đã, rồi hẵng viết script — không thì script của bạn cũng đổi theo mỗi tuần.

> Khi thiết kế thay đổi — và nó luôn thay đổi — chạy lại script mất hai phút. Dựng tay mất hai tuần. Chọn đi.

Trên dự án thoát nước gần đây, cả mạng lưới 84 hố ga được đặt tự động từ bảng tính, và mỗi lần điều chỉnh cao độ chỉ tốn một lần chạy script. Cái giá duy nhất là thời gian viết script ban đầu — cái lãi là mọi lần thay đổi sau đó. Với nghề của mình, đó là khoản đầu tư lãi kép rõ ràng nhất.
