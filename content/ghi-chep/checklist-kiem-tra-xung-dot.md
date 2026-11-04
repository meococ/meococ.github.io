+++
title = "Checklist kiểm tra xung đột cho buổi họp phối hợp — Huỳnh Trung"
h1 = "Checklist kiểm tra xung đột cho buổi họp phối hợp"
description = "Phần lớn clash không phải lỗi dựng hình mà là lỗi giao tiếp. Một checklist thực dụng để buổi họp phối hợp BIM kết thúc đúng giờ và ra được quyết định."
kind = "post"
section = "writing"
read_bar = true
rail = "HUỲNH TRUNG — GHI CHÉP"
foot_tag = "GHI CHÉP · BIM & AEC"
kicker = "GHI CHÉP · BIM & AEC"
date = "2026.10"
minutes = "4 PHÚT ĐỌC"
tag = "BIM & AEC"
back_href = "/#writing"
back_label = "TẤT CẢ GHI CHÉP"
nav = [
  { label = "BÀI TIẾP THEO →", href = "/ghi-chep/dynamo-dung-mot-lan-cap-nhat-moi-noi.html", title = "Dynamo: dựng một lần, cập nhật mọi nơi" },
  { label = "XEM →", href = "/#projects", title = "Các dự án của mình" },
]
+++

Sau vài năm ngồi ghế điều phối, mình nhận ra một điều hơi khó chấp nhận: **phần lớn clash không phải lỗi dựng hình — mà là lỗi giao tiếp**. Hai bộ môn cùng làm đúng theo bản vẽ của mình, rồi gặp nhau trong Navisworks thành xung đột. Vấn đề nằm ở chỗ không ai nói với ai.

Vì vậy, buổi họp phối hợp tốt không bắt đầu lúc mọi người ngồi vào bàn. Nó bắt đầu từ lúc chạy clash test. Đây là checklist mình đang dùng — bạn lấy về sửa theo dự án của mình nhé.

## Trước buổi họp — chuẩn bị thế nào

- **Chạy clash test cố định mốc thời gian** — ví dụ 16:00 thứ Năm hàng tuần, ai cũng biết dữ liệu lấy từ thời điểm nào.
- **Lọc theo nhóm nghiêm trọng**: hard clash giữa kết cấu — MEP đi trước; khoảng cách hở (clearance) đi sau. Đừng để 2.000 dòng noise che mất 30 vấn đề thật.
- **Gán chủ sở hữu ngay trong report** — mỗi clash phải có tên người chịu trách nhiệm trước khi ai đó hỏi "cái này của bên nào?".
- **Gửi danh sách rút gọn trước 24 giờ** — chỉ những clash cần quyết định tại bàn. Không ai đọc 50 trang trong buổi họp.

## Trong buổi họp — ra quyết định, không tiếp vấn đề

- Mỗi clash chỉ có một câu hỏi: **di chuyển bên nào, đi đâu, bao giờ xong**. Hết ba câu này là sang mục tiếp theo.
- Clash nào mất quá 5 phút không quyết được → **chuyển thành issue có deadline**, hai bên tự gặp riêng. Đừng lấy thời gian của 10 người trả giá cho 1 vấn đề.
- Ghi **quyết định, không ghi ý kiến**. "Bên MEP sẽ hạ cao độ cáp 30cm, hoàn thành thứ Sáu" — chứ không phải "có lẽ nên xem lại".

> Buổi họp phối hợp tốt nhất là buổi họp mà người ta không cần tới — vì mọi thứ đã được xử lý trước đó.

## Sau buổi họp — khép vòng lặp

- Cập nhật nhật ký quyết định trong **24 giờ** — trong lúc mọi người còn nhớ mình đã đồng ý cái gì.
- Chạy lại clash test đầu tuần sau để **xác nhận các quyết định đã được thực hiện** trong mô hình, không chỉ trên miệng.
- Đếm số clash tái xuất hiện — nếu cùng một chỗ xung đột lại lần hai, vấn đề không nằm ở mô hình nữa, mà ở quy trình.

Checklist này không phải chân lý gì — nó chỉ là cách mình biến "kiểm tra xung đột" từ công việc máy móc thành **một thói quen giao tiếp**. Và theo mình, đó mới là công việc thật của người điều phối BIM.
