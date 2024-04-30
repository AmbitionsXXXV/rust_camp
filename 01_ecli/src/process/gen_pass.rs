// 引入随机化函数库中的切片随机选择功能
use rand::seq::SliceRandom;

// 定义常量数组，分别用于大写字母、小写字母、数字和符号字符集
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

/// 生成具有特定要求的随机密码
///
/// 参数:
/// * `length`: 密码长度
/// * `upper`: 是否包含大写字母
/// * `lower`: 是否包含小写字母
/// * `number`: 是否包含数字
/// * `symbol`: 是否包含特殊符号
///
/// 返回:
/// * 返回生成的密码字符串，若生成失败则返回错误
pub fn process_gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng(); // 创建随机数生成器
    let mut password = Vec::new(); // 用于存储最终密码字符的向量
    let mut chars = Vec::new(); // 用于存储所有可能的密码字符

    // 根据函数参数决定是否将特定字符集添加到可选字符向量中，并保证至少有一个字符被选择
    if upper {
        chars.extend_from_slice(UPPER); // 将大写字母添加到字符集
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty")); // 确保密码中至少有一个大写字母
    }
    if lower {
        chars.extend_from_slice(LOWER); // 将小写字母添加到字符集
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty")); // 确保密码中至少有一个小写字母
    }
    if number {
        chars.extend_from_slice(NUMBER); // 将数字添加到字符集
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
        // 确保密码中至少有一个数字
    }
    if symbol {
        chars.extend_from_slice(SYMBOL); // 将符号添加到字符集
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
        // 确保密码中至少有一个符号
    }

    // 如果没有选择任何字符集，则返回错误
    if chars.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one character set must be selected"
        ));
    }

    // 填充密码至所需长度
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context"); // 从所有可用字符中随机选择
        password.push(*c);
    }

    password.shuffle(&mut rng); // 随机化密码中字符的顺序

    Ok(String::from_utf8(password)?) // 将密码向量转换为字符串并返回
}
