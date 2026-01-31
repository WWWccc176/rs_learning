use plotters::prelude::*;

// 优化后的素数判断函数
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num.is_multiple_of(2) {
        return false;
    }
    let limit = (num as f64).sqrt() as u64;

    for i in (3..=limit).step_by(2) {
        if num.is_multiple_of(i) {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_x: u64 = 10000;

    // 准备数据
    let mut data: Vec<(f32, f32)> = Vec::new();
    let mut prime_count: u64 = 0;

    for x in 0..=max_x {
        if is_prime(x) {
            prime_count += 1;
        }
        data.push((x as f32, prime_count as f32));
    }

    // --- 开始绘图 ---
    let root = BitMapBackend::new("outputs/prime_plot.png", (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("素数计数函数 π(x)", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..10000f32, 0f32..1400f32)?;

    chart.configure_mesh().draw()?;

    // --- 消除警告的关键修改 ---
    // 下面的 RED, BLACK 等颜色，直接传值即可，不需要加 '&'

    chart
        .draw_series(LineSeries::new(
            data.iter().map(|&(x, y)| (x, y)), // 使用迭代器
            RED.stroke_width(2),               // 指定线宽
        ))?
        .label("y = π(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED)); // 修改点：直接使用 RED

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8)) // 修改点：直接传值，去掉 &
        .border_style(BLACK) // 修改点：直接传值，去掉 &
        .draw()?;

    root.present()?;

    println!("绘制完成！请查看: prime_plot.png");

    Ok(())
}
