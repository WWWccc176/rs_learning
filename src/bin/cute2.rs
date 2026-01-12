use image::{GenericImageView, Rgba, imageops::FilterType};
use std::env;

fn main() {
    // 1. 获取图片路径 (这里为了方便，如果没有参数默认读取 'cute.jpg')
    let args: Vec<String> = env::args().collect();
    let img_path = if args.len() > 1 {
        &args[1]
    } else {
        "images/cute2.jpg"
    };

    println!("正在处理图片: {}", img_path);

    // 2. 加载图片
    let img = match image::open(img_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("无法打开图片. 请确保文件存在. 错误: {}", e);
            return;
        }
    };

    // 3. 设置输出宽度
    // 终端一行通常在 80-120 字符左右。
    // 这张图比较宽，我们设置宽度为 80，高度会自动计算。
    let target_width = 80;

    // 计算目标高度。
    // 注意：因为我们用 "▀" 字符，一个字符包含垂直的2个像素。
    // 所以我们在缩放时，要保持原本的宽高比，不用像普通ASCII那样把高度减半。
    let (orig_w, orig_h) = img.dimensions();
    let aspect_ratio = orig_h as f64 / orig_w as f64;
    let target_height = (target_width as f64 * aspect_ratio) as u32;

    // 4. 高质量缩放图片 (Lanczos3 算法能保持线条清晰，适合动漫风格)
    let scaled = img.resize_exact(target_width, target_height, FilterType::Lanczos3);

    // 5. 生成字符画字符串
    let art_string = generate_utf8_art(&scaled, target_width, target_height);

    // 6. 打印出来
    println!("{}", art_string);
}

fn generate_utf8_art(img: &image::DynamicImage, width: u32, height: u32) -> String {
    let mut result = String::with_capacity((width * height * 20) as usize); // 预分配内存

    // 每次处理两行像素 (y 和 y+1)
    for y in (0..height).step_by(2) {
        for x in 0..width {
            // 获取上半部分像素 (前景色)
            let top_pixel = img.get_pixel(x, y);

            // 获取下半部分像素 (背景色)
            // 处理边界：如果是最后一行且是奇数行，下面没有像素了，就设为透明/黑色
            let bottom_pixel = if y + 1 < height {
                img.get_pixel(x, y + 1)
            } else {
                Rgba([0, 0, 0, 0])
            };

            // 拼接 ANSI 转义序列
            // \x1b[38;2;R;G;Bm 设置前景色 (Top)
            // \x1b[48;2;R;G;Bm 设置背景色 (Bottom)
            // ▀ 字符利用前景色填满上半格，背景色填满下半格
            let chunk = format!(
                "\x1b[38;2;{r1};{g1};{b1}m\x1b[48;2;{r2};{g2};{b2}m▀",
                r1 = top_pixel[0],
                g1 = top_pixel[1],
                b1 = top_pixel[2],
                r2 = bottom_pixel[0],
                g2 = bottom_pixel[1],
                b2 = bottom_pixel[2]
            );
            result.push_str(&chunk);
        }
        // 每一行结束重置颜色并换行
        result.push_str("\x1b[0m\n");
    }

    result
}
