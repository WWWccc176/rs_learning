use image::{ImageBuffer, Rgb, RgbImage};
use rand::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

// 定义方向：上、下、左、右
const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Maze {
    h: usize,
    w: usize,
    // 墙壁数据：true表示有墙，false表示打通
    u: Vec<Vec<bool>>,
    d: Vec<Vec<bool>>,
    l: Vec<Vec<bool>>,
    r: Vec<Vec<bool>>,
}

impl Maze {
    fn new(h: usize, w: usize) -> Self {
        Maze {
            h,
            w,
            u: vec![vec![true; w]; h],
            d: vec![vec![true; w]; h],
            l: vec![vec![true; w]; h],
            r: vec![vec![true; w]; h],
        }
    }
}

/// Wilson 算法生成迷宫
/// 返回：(迷宫结构体, 起点, 终点)
fn generate_maze_wilson(h: usize, w: usize) -> (Maze, (usize, usize), (usize, usize)) {
    let mut rng = thread_rng();
    let mut maze = Maze::new(h, w);

    // 1. 访问标记 (true = 未访问, false = 已访问)
    // 注意：Rust 初始化 bool 数组，这里为了逻辑清晰，我们用 visited: bool, true 代表已加入树
    let mut in_maze = vec![vec![false; w]; h];

    // 入口 (0,0)
    let start = (0, 0);
    in_maze[start.0][start.1] = true;

    // 剩余未访问格子数
    let mut unvisited_count = h * w - 1;

    // 进度条 (可选)
    println!("Generating Maze ({}x{})...", h, w);

    // 收集所有未访问的坐标，方便随机选取
    // 优化：虽然可以维护一个列表，但随着迷宫变大，随机取坐标并判断是否在迷宫内更简单
    while unvisited_count > 0 {
        // A. 随机选一个还不在迷宫中的点作为起点
        let curr;
        loop {
            let ry = rng.gen_range(0..h);
            let rx = rng.gen_range(0..w);
            if !in_maze[ry][rx] {
                curr = (ry, rx);
                break;
            }
        }

        // B. Loop-Erased Random Walk (LERW)
        // 路径记录，用于消除环
        let mut path = vec![curr];

        loop {
            let (cr, cc) = *path.last().unwrap();

            // 如果撞到了迷宫主体（已访问区域），结束游走
            if in_maze[cr][cc] {
                break;
            }

            // 随机走一步
            let (dr, dc) = DIRS[rng.gen_range(0..4)];
            let nr = cr as isize + dr;
            let nc = cc as isize + dc;

            // 边界检查
            if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                continue;
            }
            let next_node = (nr as usize, nc as usize);

            // C. 消除环 (Loop Erasure)
            if let Some(pos) = path.iter().position(|&x| x == next_node) {
                // 如果新点已经在路径里，截断路径到该点位置
                path.truncate(pos + 1);
            } else {
                path.push(next_node);
            }
        }

        // D. 将路径上的墙打通，并标记为已访问
        for window in path.windows(2) {
            let (r1, c1) = window[0];
            let (r2, c2) = window[1];

            // 只要不是最后一点（最后一点是撞入迷宫的点），都算作新加入的点
            if !in_maze[r1][c1] {
                in_maze[r1][c1] = true;
                unvisited_count -= 1;
            }

            // 如果撞入的点之前不在迷宫里（理论上最后一步必定撞入in_maze为true的点，
            // 但路径里的前一个点还没设为true），这里不需要额外处理r2,c2的计数，
            // 因为下一轮或者之前已经处理过。我们只负责打墙。

            if r2 as isize == r1 as isize - 1 && c2 == c1 {
                // 上
                maze.u[r1][c1] = false;
                maze.d[r2][c2] = false;
            } else if r2 as isize == r1 as isize + 1 && c2 == c1 {
                // 下
                maze.d[r1][c1] = false;
                maze.u[r2][c2] = false;
            } else if r2 == r1 && c2 as isize == c1 as isize - 1 {
                // 左
                maze.l[r1][c1] = false;
                maze.r[r2][c2] = false;
            } else if r2 == r1 && c2 as isize == c1 as isize + 1 {
                // 右
                maze.r[r1][c1] = false;
                maze.l[r2][c2] = false;
            }
        }
    }

    // 设置固定出口 (H-1, W-2)
    let exit_pos = (h - 1, w - 2);
    // 这里Python代码只是在grid上画了口子，逻辑上的 U/D/L/R 没有通向迷宫外的墙，
    // 但作为迷宫内部结构，(H-1, W-2) 是存在的。
    // 我们原样返回。

    (maze, start, exit_pos)
}

/// CPU BFS 计算最短路
fn bfs_cpu(maze: &Maze, start: (usize, usize)) -> Vec<Vec<i32>> {
    let h = maze.h;
    let w = maze.w;
    let mut dist = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();

    dist[start.0][start.1] = 0;
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        let d = dist[r][c];

        // 尝试 4 个方向，检查是否有墙

        // 上 (-1, 0) -> 检查 u
        if r > 0 && !maze.u[r][c] {
            let nr = r - 1;
            let nc = c;
            if dist[nr][nc] == -1 {
                dist[nr][nc] = d + 1;
                queue.push_back((nr, nc));
            }
        }
        // 下 (1, 0) -> 检查 d
        if r < h - 1 && !maze.d[r][c] {
            let nr = r + 1;
            let nc = c;
            if dist[nr][nc] == -1 {
                dist[nr][nc] = d + 1;
                queue.push_back((nr, nc));
            }
        }
        // 左 (0, -1) -> 检查 l
        if c > 0 && !maze.l[r][c] {
            let nr = r;
            let nc = c - 1;
            if dist[nr][nc] == -1 {
                dist[nr][nc] = d + 1;
                queue.push_back((nr, nc));
            }
        }
        // 右 (0, 1) -> 检查 r
        if c < w - 1 && !maze.r[r][c] {
            let nr = r;
            let nc = c + 1;
            if dist[nr][nc] == -1 {
                dist[nr][nc] = d + 1;
                queue.push_back((nr, nc));
            }
        }
    }

    dist
}

/// 渲染结果为图片
fn render_maze_image(
    maze: &Maze,
    dist: &Vec<Vec<i32>>,
    start: (usize, usize),
    exit_pos: (usize, usize),
    filename: &str,
) {
    let h = maze.h;
    let w = maze.w;
    // 栅格图大小：每个单元格占 2x2 像素区域 (其实是 2H+1 x 2W+1)
    let img_h = (2 * h + 1) as u32;
    let img_w = (2 * w + 1) as u32;

    let mut img: RgbImage = ImageBuffer::new(img_w, img_h);

    // 1. 先把背景涂黑 (墙)
    for pixel in img.pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }

    // 找到最大距离用于归一化颜色 (BFS的最远端)
    let mut max_dist = 1;
    for r in 0..h {
        for c in 0..w {
            if dist[r][c] > max_dist {
                max_dist = dist[r][c];
            }
        }
    }

    // 2. 绘制迷宫内部
    for r in 0..h {
        for c in 0..w {
            let d = dist[r][c];

            // 计算颜色 (简单的热力图: 黑 -> 红 -> 黄 -> 白)
            // 这里为了简单，我们用 Pink 风格 (类似 matplotlib pink)
            // 距离越远，颜色越亮/暖
            let color = if d == -1 {
                Rgb([0, 0, 0]) // 不可达（理论上Wilson全连通，除非死路）
            } else {
                let ratio = d as f32 / max_dist as f32;
                // 简单的渐变：深红 -> 浅粉/白
                // R: 随着距离增加增加
                // G: 增加较慢
                // B: 增加较慢
                // 这只是模拟，不完全等同于 matplotlib 'pink'
                let red = (50.0 + 205.0 * ratio) as u8;
                let green = (20.0 + 200.0 * ratio * ratio) as u8; // 平方让高光集中
                let blue = (20.0 + 200.0 * ratio * ratio) as u8;
                Rgb([red, green, blue])
            };

            // 转换到 grid 坐标
            // 单元格中心在 (2r+1, 2c+1)
            let center_y = 2 * r + 1;
            let center_x = 2 * c + 1;

            // 绘制中心点
            img.put_pixel(center_x as u32, center_y as u32, color);

            // 绘制连通的通道 (打通墙壁)
            // 向右画
            if !maze.r[r][c] {
                img.put_pixel(center_x as u32 + 1, center_y as u32, color);
            }
            // 向下画
            if !maze.d[r][c] {
                img.put_pixel(center_x as u32, center_y as u32 + 1, color);
            }
            // 上和左其实由前一个格子的 下和右 覆盖了，但为了完整性，
            // 只需要画右和下即可填充完整个图，因为是双向的。
            // 只要确保中心点和连接点颜色一致即可。
        }
    }

    // 3. 处理入口和出口的墙壁 (手动打通边界)
    // 入口 (1, 0) 对应的 grid 坐标. 在Python代码中 grid[1,0]=0
    // 逻辑坐标 start=(0,0). 2*0+1=1 (y), 2*0+1=1 (x). 左边是 x=0.
    img.put_pixel(0, 1, Rgb([0, 255, 0])); // 绿色标示入口
    img.put_pixel(1, 1, Rgb([0, 255, 0]));

    // 出口 (H-1, W-2). Python: grid[2*ey+1, 2*ex+2] = 0 (右边)
    let (ey, ex) = exit_pos;
    let exit_y = (2 * ey + 1) as u32;
    let exit_x = (2 * ex + 1) as u32; // 这是格子中心
    // 打通右边的墙
    img.put_pixel(exit_x + 1, exit_y, Rgb([255, 0, 0])); // 红色标示出口
    img.put_pixel(exit_x, exit_y, Rgb([255, 0, 0]));

    println!("Saving image to {}...", filename);
    img.save(Path::new(filename)).unwrap();
}

fn main() {
    let h = 300;
    let w = 300;

    // 1. 生成
    let (maze, start, exit_pos) = generate_maze_wilson(h, w);

    // 2. 求解 (CPU BFS)
    println!("Solving Maze (BFS)...");
    let dist = bfs_cpu(&maze, start);

    // 3. 绘图
    render_maze_image(&maze, &dist, start, exit_pos, "maze_rust.png");

    println!("Done! Check maze_rust.png");
}
