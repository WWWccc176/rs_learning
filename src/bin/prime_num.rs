use plotters::prelude::*;

// 优化后的素数判断函数
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    // 单独处理 2，它是唯一的偶数素数
    if num == 2 {
        return true;
    }
    // 排除掉所有偶数
    if num.is_multiple_of(2) {
        return false;
    }

    // 计算平方根上限
    let limit = (num as f64).sqrt() as u64;

    // 从 3 开始，步长为 2 (只检查奇数：3, 5, 7...)
    // 这样效率更高，且符合 Rust 常见写法
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
    let root = BitMapBackend::new("prime_plot.png", (50000, 37500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("素数计数函数 π(x)", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..10000f32, 0f32..10000f32)?;

    chart.configure_mesh().draw()?;

    // --- 消除警告的关键修改 ---
    // 下面的 RED, BLACK 等颜色，直接传值即可，不需要加 '&'

    chart
        .draw_series(LineSeries::new(
            data, RED, // 修改点：直接使用 RED，去掉 &
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
