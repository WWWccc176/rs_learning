use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// -----------------------------------------------------------
// 1. 自制随机数生成器 (LCG算法)
//    因为不能用 rand 库，我们必须自己造一个
// -----------------------------------------------------------
struct AsciiRng {
    state: u64,
}

impl AsciiRng {
    // 初始化种子
    fn new() -> Self {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
        AsciiRng {
            state: since_epoch.as_nanos() as u64,
        }
    }

    // 生成下一个 u64 随机数
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    // [核心逻辑]：直接生成 ASCII 可打印字符
    // ASCII 可打印范围是 32 (空格) 到 126 (~)
    fn next_ascii_char(&mut self) -> char {
        let min_ascii = 32;
        let max_ascii = 127; // 不包含127，所以范围是 32..127
        let range = max_ascii - min_ascii;

        // 随机数 % 范围大小 + 最小值 = 落在 ASCII 范围内的数值
        let ascii_val = (self.next_u64() % range) + min_ascii;

        // 强制转换为 char
        ascii_val as u8 as char
    }
}

// -----------------------------------------------------------
// 2. 主逻辑
// -----------------------------------------------------------
fn main() {
    let target = "Hello, pyjastpp112358!";
    println!("Hello, pyjastpp112358!");
    // 初始化：先用空格填充，长度与目标一致
    let mut current_chars: Vec<char> = vec![' '; target.len()];

    // 状态标记：false表示该位还在乱跳，true表示已破解锁定
    let mut locked: Vec<bool> = vec![false; target.len()];

    let mut rng = AsciiRng::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // 锁定 IO 提速

    loop {
        let mut all_done = true;

        for (i, target_char) in target.chars().enumerate() {
            // 如果这一位已经破解了，就跳过，保持原样
            if locked[i] {
                current_chars[i] = target_char;
                continue;
            }

            // [重点]：这一位还没破解，直接从 ASCII 码全集中随机生成一个字符
            let random_char = rng.next_ascii_char();
            current_chars[i] = random_char;

            // 检查是否撞库成功
            if random_char == target_char {
                locked[i] = true;
            } else {
                all_done = false; // 只要有一位没对上，就不能结束
            }
        }

        // 打印输出：\r 用于回车不换行，覆盖上一行
        let output: String = current_chars.iter().collect();
        write!(handle, "\r{}", output).unwrap();
        handle.flush().unwrap();

        if all_done {
            break;
        }

        // 控制刷新率，数字越小跳动越快
        thread::sleep(Duration::from_millis(15));
    }

    println!("\n\nPassword Accepted.");
}
