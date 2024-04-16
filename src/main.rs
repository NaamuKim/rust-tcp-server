use std::f64::consts::PI;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let delay = Duration::from_millis(100);
    let mut angle = 0.0;

    loop {
        // 각도를 라디안으로 변환
        let radian = angle * PI / 180.0;

        // 사각형의 네 꼭지점 정의
        let points = vec![
            (5.0, 5.0),   // 상단 왼쪽
            (5.0, -5.0),  // 상단 오른쪽
            (-5.0, -5.0), // 하단 오른쪽
            (-5.0, 5.0),  // 하단 왼쪽
        ];

        // 회전 행렬을 사용하여 점들 회전
        let rotated_points: Vec<(f64, f64)> = points
            .iter()
            .map(|&(x, y)| {
                let rotated_x = x * radian.cos() - y * radian.sin();
                let rotated_y = x * radian.sin() + y * radian.cos();
                (rotated_x, rotated_y)
            })
            .collect();

        // 화면을 지우고 새 위치에 사각형 그리기
        write!(stdout, "\x1B[2J").unwrap(); // 화면 클리어
        for i in 0..rotated_points.len() {
            let (start, end) = (
                rotated_points[i],
                rotated_points[(i + 1) % rotated_points.len()],
            );
            draw_line(start, end, &mut stdout);
        }
        stdout.flush().unwrap();

        // 각도 증가
        angle += 5.0;
        if angle >= 360.0 {
            angle = 0.0;
        }

        // 딜레이
        sleep(delay);
    }
}

// 선 그리기 함수 (간단한 선 알고리즘 사용)
fn draw_line(mut start: (f64, f64), end: (f64, f64), stdout: &mut std::io::Stdout) {
    let dx = (end.0 - start.0).abs();
    let dy = -(end.1 - start.1).abs();
    let sx = if start.0 < end.0 { 1.0 } else { -1.0 };
    let sy = if start.1 < end.1 { 1.0 } else { -1.0 };
    let mut err = dx + dy;

    loop {
        // 터미널의 중앙에 위치 조정
        let x = (40.0 + start.0) as u16;
        let y = (12.0 + start.1) as u16;
        write!(stdout, "\x1B[{};{}H&", y, x).unwrap();

        if start.0 == end.0 && start.1 == end.1 {
            break;
        }
        let e2 = 2.0 * err;
        if e2 >= dy {
            if start.0 == end.0 {
                break;
            }
            err += dy;
            start.0 += sx;
        }
        if e2 <= dx {
            if start.1 == end.1 {
                break;
            }
            err += dx;
            start.1 += sy;
        }
    }
}
