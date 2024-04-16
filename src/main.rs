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

        // 선의 끝점 계산
        let endpoint1 = (10.0, 0.0); // 원점에서 오른쪽으로 10 단위
        let endpoint2 = (-10.0, 0.0); // 원점에서 왼쪽으로 10 단위

        // 회전 행렬을 사용하여 두 점 회전
        let rotated_x1 = endpoint1.0 * radian.cos() - endpoint1.1 * radian.sin();
        let rotated_y1 = endpoint1.0 * radian.sin() + endpoint1.1 * radian.cos();
        let rotated_x2 = endpoint2.0 * radian.cos() - endpoint2.1 * radian.sin();
        let rotated_y2 = endpoint2.0 * radian.sin() + endpoint2.1 * radian.cos();

        // 터미널 중앙에 위치 조정
        let x1 = (40.0 + rotated_x1) as u16;
        let y1 = (12.0 + rotated_y1) as u16;
        let x2 = (40.0 + rotated_x2) as u16;
        let y2 = (12.0 + rotated_y2) as u16;

        // 화면을 지우고 새 위치에 선 그리기
        write!(stdout, "\x1B[2J\x1B[{};{}H*\x1B[{};{}H*", y1, x1, y2, x2).unwrap();
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
