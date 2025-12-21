use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use std::thread;
use std::time::Duration;

// --- 配置参数 ---
const H: usize = 100; // 迷宫高度 (格子数)
const W: usize = 100; // 迷宫宽度
const SCALE: usize = 4; // 放大倍数，每个像素点在屏幕上显示为 4x4，否则太小看不清

// 屏幕(Grid)尺寸: (2H+1) x (2W+1)
const GRID_H: usize = 2 * H + 1;
const GRID_W: usize = 2 * W + 1;

// 颜色定义 (0x00RRGGBB)
const COLOR_WALL: u32 = 0x000000; // 黑
const COLOR_PATH: u32 = 0xFFFFFF; // 白
const COLOR_TEMP: u32 = 0xFF0000; // 红 (正在寻找的路径)
const COLOR_START: u32 = 0x00FF00; // 绿 (入口)
const COLOR_EXIT: u32 = 0x0000FF; // 蓝 (出口)

fn main() {
    // 1. 初始化窗口
    let window_w = GRID_W * SCALE;
    let window_h = GRID_H * SCALE;

    let mut window = Window::new(
        "Wilson's Algorithm Maze Generation - Rust",
        window_w,
        window_h,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // 限制帧率，大约 60fps，防止风扇狂转
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // 2. 像素缓冲区 (存储当前画面的颜色)
    // 逻辑大小是 GRID_W * GRID_H，我们最后渲染时再放大
    // 为了简化，我们直接建立一个跟窗口像素 1:1 的 buffer
    let mut buffer: Vec<u32> = vec![COLOR_WALL; window_w * window_h];

    // 3. 迷宫数据结构
    // grid[y][x] 存储颜色。我们用一个小的 grid 来逻辑绘图，然后映射到 buffer
    let mut grid_colors = vec![vec![COLOR_WALL; GRID_W]; GRID_H];

    // 初始化：奇数行列是路(虽然Wilson初始全是墙，但为了格子对其，先把中心点留出来)
    // 实际上 Wilson 初始全是墙。我们把 (1,1) 这种格子中心逻辑上看作节点。

    // 打开入口
    set_grid_color(&mut grid_colors, 1, 0, COLOR_START);

    // 4. Wilson 算法状态
    let mut rng = thread_rng();
    let mut in_maze = vec![vec![false; W]; H];
    let start = (0, 0);
    in_maze[start.0][start.1] = true;
    set_grid_color(&mut grid_colors, 1, 1, COLOR_PATH); // 标记起点已通

    let mut unvisited_count = H * W - 1;
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // --- 动画循环 ---
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // 如果迷宫还没生成完，继续跑算法
        if unvisited_count > 0 {
            // A. 找一个不在迷宫的点
            let mut curr;
            loop {
                let ry = rng.gen_range(0..H);
                let rx = rng.gen_range(0..W);
                if !in_maze[ry][rx] {
                    curr = (ry, rx);
                    break;
                }
            }

            // B. 随机游走 (Loop-Erased Random Walk)
            let mut path = vec![curr];

            // 为了让动画可见，我们每走一步更新一次画面（或者每几步）
            // 注意：因为 random walk 可能很长，为了性能，我们这里不每一步都刷新窗口，
            // 而是把一次完整的 "walk until hit" 做完再刷新，或者在 walk 内部刷新。
            // 为了好看，我们只在 "打通墙壁" 的时候刷新（类似 Python 代码的效果）。

            loop {
                let (cr, cc) = *path.last().unwrap();
                if in_maze[cr][cc] {
                    break;
                } // 撞墙了

                let (dr, dc) = dirs[rng.gen_range(0..4)];
                let nr = cr as isize + dr;
                let nc = cc as isize + dc;

                if nr >= 0 && nr < H as isize && nc >= 0 && nc < W as isize {
                    let next_node = (nr as usize, nc as usize);
                    if let Some(pos) = path.iter().position(|&x| x == next_node) {
                        path.truncate(pos + 1); // 消除环
                    } else {
                        path.push(next_node);
                    }
                }
            }

            // C. 路径确立，打通墙壁，并更新显示
            for window_slice in path.windows(2) {
                let (r1, c1) = window_slice[0];
                let (r2, c2) = window_slice[1];

                if !in_maze[r1][c1] {
                    in_maze[r1][c1] = true;
                    unvisited_count -= 1;

                    // 绘制节点 (2*r+1, 2*c+1)
                    set_grid_color(&mut grid_colors, 2 * r1 + 1, 2 * c1 + 1, COLOR_PATH);
                }

                // 绘制两个节点中间的墙
                // r1,c1 和 r2,c2 中间坐标：
                let wall_r = (2 * r1 + 1) as isize + (r2 as isize - r1 as isize);
                let wall_c = (2 * c1 + 1) as isize + (c2 as isize - c1 as isize);
                set_grid_color(
                    &mut grid_colors,
                    wall_r as usize,
                    wall_c as usize,
                    COLOR_PATH,
                );
            }

            // 确保最后一个点（撞入迷宫的点）也被画亮（如果是刚连上的话）
            // 但其实它已经是 true，所以颜色应该是 COLOR_PATH。

            // --- 每一条路径生成后，更新屏幕 ---
            // 将 grid_colors 渲染到 buffer
            render_grid_to_buffer(&grid_colors, &mut buffer, window_w);

            // 更新窗口
            window
                .update_with_buffer(&buffer, window_w, window_h)
                .unwrap();

            // 如果觉得太快，可以取消下面这行的注释
            // thread::sleep(Duration::from_millis(5));
        } else {
            // 生成完成！绘制出口
            // 出口 (H-1, W-2) -> 右边开口
            let (ey, ex) = (H - 1, W - 2);
            set_grid_color(&mut grid_colors, 2 * ey + 1, 2 * ex + 2, COLOR_EXIT);

            render_grid_to_buffer(&grid_colors, &mut buffer, window_w);
            window
                .update_with_buffer(&buffer, window_w, window_h)
                .unwrap();

            println!("迷宫生成完毕！按 ESC 退出。");

            // 保持窗口打开，直到按下 ESC
            while window.is_open() && !window.is_key_down(Key::Escape) {
                window
                    .update_with_buffer(&buffer, window_w, window_h)
                    .unwrap();
            }
            break;
        }
    }
}

// 辅助函数：设置 Grid 颜色
fn set_grid_color(grid: &mut Vec<Vec<u32>>, r: usize, c: usize, color: u32) {
    if r < grid.len() && c < grid[0].len() {
        grid[r][c] = color;
    }
}

// 辅助函数：把 Grid 放大并拷贝到 Framebuffer
fn render_grid_to_buffer(grid: &Vec<Vec<u32>>, buffer: &mut Vec<u32>, win_w: usize) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let color = grid[r][c];

            // 绘制 SCALE x SCALE 的矩形
            for sy in 0..SCALE {
                for sx in 0..SCALE {
                    let buf_y = r * SCALE + sy;
                    let buf_x = c * SCALE + sx;

                    let idx = buf_y * win_w + buf_x;
                    if idx < buffer.len() {
                        buffer[idx] = color;
                    }
                }
            }
        }
    }
}
