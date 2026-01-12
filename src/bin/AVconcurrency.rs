use plotters::prelude::*;
use rayon::prelude::*; // 引入并行迭代器
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 设置参数 n
    let n_list: Vec<usize> = (30..=1000).step_by(2).collect();

    // 2. 准备评估点 s (只读数据，线程间共享)
    // 使用 capacity 避免重分配
    let mut s = Vec::with_capacity(401);
    let mut curr = -10.0;
    while curr <= 10.0 + 1e-10 {
        s.push(curr);
        curr += 0.05;
    }
    // 强制变为不可变引用，方便闭包捕获
    let s = s;
    let m_points = s.len();

    // 目标函数
    let func = |x: f64| (-x.powi(2) / 20.0).exp() * (5.0 * x).cos();

    // 真实值 (只读，线程间共享)
    let f_true: Vec<f64> = s.iter().map(|&val| func(val)).collect();

    println!(
        "Starting parallel calculation on {} threads...",
        rayon::current_num_threads()
    );
    println!("{:<5} | {:<15}", "n", "Max Error");
    println!("{}", "-".repeat(25));

    // --- 3. 并行计算核心 ---
    // 使用 par_iter() 替代 iter()
    // map() 替代 for 循环，结果自动收集
    let mut results: Vec<(usize, f64)> = n_list
        .par_iter()
        .map(|&n| {
            // --- 核心算法 (完全独立，无副作用) ---
            let (d, h, _) = arnoldi_cheb_vand_fit(func, n);

            // --- 评估阶段 ---
            // 预分配内存，减少 realloc
            let mut w_cols = Vec::with_capacity(n + 1);
            w_cols.push(vec![1.0; m_points]);

            for k in 0..n {
                // w = s .* W(:,k)
                // 这里的 zip map collect 是比较耗时的，但为了并行安全保持现状即可
                let mut w: Vec<f64> = s.iter().zip(&w_cols[k]).map(|(si, wik)| si * wik).collect();

                // 正交化修正 (MGS)
                for j in 0..=k {
                    let h_val = h[k][j];
                    // 使用迭代器避免边界检查
                    for (val, &col_val) in w.iter_mut().zip(&w_cols[j]) {
                        *val -= h_val * col_val;
                    }
                }

                // 归一化
                let h_next = h[k][k + 1];
                let inv_h_next = 1.0 / h_next; // 乘法比除法快
                for val in w.iter_mut() {
                    *val *= inv_h_next;
                }
                w_cols.push(w);
            }

            // 计算拟合值 f_fit = W * d
            let mut f_fit = vec![0.0; m_points];
            for (col_idx, col_vec) in w_cols.iter().enumerate() {
                let coef = d[col_idx];
                // 简单的循环展开提示
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

            // 返回元组 (n, error)
            (n, max_err)
        })
        .collect();

    // 并行计算会打乱顺序，需要按 n 重新排序
    results.sort_by(|a, b| a.0.cmp(&b.0));

    // --- 输出与数据准备 ---
    let mut plot_data = Vec::with_capacity(results.len());
    for (n, err) in results {
        println!("{:<5} | {:<15.8e}", n, err);
        if err > 0.0 {
            plot_data.push((n as f64, err));
        }
    }

    // --- 4. 画图 ---
    println!("\nGenerating plot: loglog_error.png ...");
    draw_loglog_plot(&plot_data)?;
    println!("Done.");

    Ok(())
}

// 绘图函数保持不变...
fn draw_loglog_plot(data: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("outputs/loglog_error_par.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let min_x = data.first().unwrap().0;
    let max_x = data.last().unwrap().0;
    let min_y = data
        .iter()
        .map(|v| v.1)
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max_y = data
        .iter()
        .map(|v| v.1)
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Error of Degree (Rust + Rayon)",
            ("sans-serif", 30).into_font(),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (min_x..max_x * 1.05).log_scale(),
            (min_y * 0.9..max_y * 1.5).log_scale(),
        )?;

    chart
        .configure_mesh()
        .x_desc("n")
        .y_desc("Max Error")
        .draw()?;
    chart.draw_series(LineSeries::new(data.iter().copied(), &BLUE))?;
    chart.draw_series(PointSeries::of_element(
        data.iter().copied(),
        3,
        &BLUE,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    root.present()?;
    Ok(())
}

fn arnoldi_cheb_vand_fit(
    func: impl Fn(f64) -> f64 + Sync, // 注意：闭包需要实现 Sync 才能跨线程
    n: usize,
) -> (Vec<f64>, Vec<Vec<f64>>, Vec<f64>) {
    let m = n + 1;
    let a = -10.0;
    let b = 10.0;

    // 预分配 x
    let mut x = Vec::with_capacity(m);
    for k in 0..m {
        let theta = PI * (k as f64) / (n as f64);
        let val = (a + b) / 2.0 + (b - a) / 2.0 * theta.cos();
        x.push(val);
    }

    // fx 计算
    let fx: Vec<f64> = x.iter().map(|&val| func(val)).collect();

    // Arnoldi 预分配
    let mut q_cols = Vec::with_capacity(m);
    q_cols.push(vec![1.0; m]);
    let mut h = Vec::with_capacity(n);

    for k in 0..n {
        let mut h_col = vec![0.0; k + 2];
        let mut q: Vec<f64> = x.iter().zip(&q_cols[k]).map(|(xi, qik)| xi * qik).collect();

        for j in 0..=k {
            // 向量点积优化
            let dot_prod: f64 = q_cols[j].iter().zip(&q).map(|(a, b)| a * b).sum();

            let h_val = dot_prod / (m as f64);
            h_col[j] = h_val;

            // 向量减法优化：使用 zip 避免索引检查
            for (val, &q_col_val) in q.iter_mut().zip(&q_cols[j]) {
                *val -= h_val * q_col_val;
            }
        }

        let norm_sq: f64 = q.iter().map(|v| v * v).sum();
        let h_next = norm_sq.sqrt() / (m as f64).sqrt();
        h_col[k + 1] = h_next;
        h.push(h_col);

        if k < n {
            let inv_h_next = 1.0 / h_next;
            let q_next: Vec<f64> = q.iter().map(|v| v * inv_h_next).collect();
            q_cols.push(q_next);
        }
    }

    let mut d = Vec::with_capacity(m);
    for col in &q_cols {
        let dot: f64 = col.iter().zip(&fx).map(|(a, b)| a * b).sum();
        d.push(dot / (m as f64));
    }

    (d, h, x)
}
