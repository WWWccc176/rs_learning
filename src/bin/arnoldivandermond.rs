use plotters::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 设置参数 n 从 30 到 1000
    let n_list: Vec<usize> = (30..=500).step_by(2).collect();

    // 2. 准备评估点 s = -10:0.05:10
    // 手动生成 grid
    let mut s = Vec::new();
    let mut curr = -10.0;
    while curr <= 10.0 + 1e-10 {
        s.push(curr);
        curr += 0.05;
    }
    let m_points = s.len();

    // 目标函数
    let func = |x: f64| (-x.powi(2) / 20.0).exp() * (5.0 * x).cos();

    // 真实值
    let f_true: Vec<f64> = s.iter().map(|&val| func(val)).collect();

    // --- 存储绘图数据 (n, error) ---
    // 我们不需要中间 CSV 文件，直接存入内存向量
    let mut plot_data: Vec<(f64, f64)> = Vec::with_capacity(n_list.len());

    println!("{:<5} | {:<15}", "n", "Max Error");
    println!("{}", "-".repeat(25));

    // 4. 循环计算
    for &n in &n_list {
        // --- 核心算法 ---
        let (d, h, _) = arnoldi_cheb_vand_fit(func, n);

        // --- 评估阶段 (Horner-like / Hessenberg evaluation) ---
        // 初始化 W 的第一列 (全1)
        let mut w_cols = Vec::with_capacity(n + 1);
        w_cols.push(vec![1.0; m_points]);

        for k in 0..n {
            // w = s .* W(:,k)
            let mut w: Vec<f64> = s.iter().zip(&w_cols[k]).map(|(si, wik)| si * wik).collect();

            // 正交化修正
            for j in 0..=k {
                let h_val = h[k][j];
                for (idx, val) in w.iter_mut().enumerate() {
                    *val -= h_val * w_cols[j][idx];
                }
            }

            // 归一化
            let h_next = h[k][k + 1];
            for val in w.iter_mut() {
                *val /= h_next;
            }
            w_cols.push(w);
        }

        // 计算拟合值 f_fit = W * d
        let mut f_fit = vec![0.0; m_points];
        for (col_idx, col_vec) in w_cols.iter().enumerate() {
            let coef = d[col_idx];
            for (row_idx, val) in col_vec.iter().enumerate() {
                f_fit[row_idx] += val * coef;
            }
        }

        // 计算最大误差
        let mut max_err = 0.0;
        for i in 0..m_points {
            let err = (f_true[i] - f_fit[i]).abs();
            if err > max_err {
                max_err = err;
            }
        }

        // --- 功能 1: 终端输出 ---
        println!("{:<5} | {:<15.8e}", n, max_err);

        // --- 收集数据用于绘图 ---
        // 注意 loglog图通常处理非零数据，防止 log(0)
        if max_err > 0.0 {
            plot_data.push((n as f64, max_err));
        }
    }

    // --- 功能 2: 画图 (Log-Log Plot) ---
    println!("\nGenerating plot: loglog_error.png ...");
    draw_loglog_plot(&plot_data)?;
    println!("Done.");

    Ok(())
}

/// 使用 plotters 绘制 Log-Log 图
fn draw_loglog_plot(data: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    // 创建绘图后端，分辨率 800x600，输出为 png 文件
    let root = BitMapBackend::new("loglog_error.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 自动寻找坐标轴范围
    let min_x = data.first().unwrap().0;
    let max_x = data.last().unwrap().0;
    // 找出 y 轴的最小值和最大值用于 Log 坐标范围
    let min_y = data
        .iter()
        .map(|v| v.1)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max_y = data
        .iter()
        .map(|v| v.1)
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));

    // 构建 Chart (Log-Log)
    // MATLAB: loglog(n_list, max_err, '*-')
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Error of Degree for Arno–Vand (Rust)",
            ("sans-serif", 30).into_font(),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        // 使用 LogCoord 实现对数坐标
        // 范围稍微扩大一点点以免点贴在边框上
        .build_cartesian_2d(
            (min_x..max_x * 1.05).log_scale(),
            (min_y * 0.9..max_y * 1.5).log_scale(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Polynomial degree n")
        .y_desc("Max absolute error")
        .x_labels(5) // 控制网格密度
        .y_labels(5)
        .draw()?;

    // 绘制线条和点 ('*-' style)
    chart.draw_series(LineSeries::new(data.iter().copied(), &BLUE))?;

    // 绘制数据点标记 (*)
    chart.draw_series(PointSeries::of_element(
        data.iter().copied(),
        3, // 点的大小
        &BLUE,
        &|c, s, st| {
            // 绘制星号或圆点
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
        },
    ))?;

    root.present()?;
    Ok(())
}

// --- 以下为数学计算函数 (保持不变) ---

fn arnoldi_cheb_vand_fit(
    func: impl Fn(f64) -> f64,
    n: usize,
) -> (Vec<f64>, Vec<Vec<f64>>, Vec<f64>) {
    let m = n + 1;
    let a = -10.0;
    let b = 10.0;

    // 1. 生成节点
    let mut x = Vec::with_capacity(m);
    for k in 0..m {
        let theta = PI * (k as f64) / (n as f64);
        let val = (a + b) / 2.0 + (b - a) / 2.0 * theta.cos();
        x.push(val);
    }
    let fx: Vec<f64> = x.iter().map(|&val| func(val)).collect();

    // 2. Arnoldi
    let mut q_cols = Vec::with_capacity(m);
    q_cols.push(vec![1.0; m]);
    let mut h = Vec::with_capacity(n);

    for k in 0..n {
        let mut h_col = vec![0.0; k + 2];
        let mut q: Vec<f64> = x.iter().zip(&q_cols[k]).map(|(xi, qik)| xi * qik).collect();

        for j in 0..=k {
            let dot_prod: f64 = q_cols[j].iter().zip(&q).map(|(a, b)| a * b).sum();

            let h_val = dot_prod / (m as f64);
            h_col[j] = h_val;

            for (idx, val) in q.iter_mut().enumerate() {
                *val -= h_val * q_cols[j][idx];
            }
        }

        let norm_sq: f64 = q.iter().map(|v| v * v).sum();
        let h_next = norm_sq.sqrt() / (m as f64).sqrt();
        h_col[k + 1] = h_next;
        h.push(h_col);

        if k < n {
            let q_next: Vec<f64> = q.iter().map(|v| v / h_next).collect();
            q_cols.push(q_next);
        }
    }

    // 3. 系数 d
    let mut d = Vec::with_capacity(m);
    for i in 0..m {
        let dot: f64 = q_cols[i].iter().zip(&fx).map(|(a, b)| a * b).sum();
        d.push(dot / (m as f64));
    }

    (d, h, x)
}
