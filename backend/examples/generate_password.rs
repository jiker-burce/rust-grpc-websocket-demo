use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let password = "system123";

    // 生成密码哈希
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => {
            println!("原始密码: {}", password);
            println!("生成的哈希: {}", hashed);

            // 验证密码
            match verify(password, &hashed) {
                Ok(valid) => {
                    println!("密码验证: {}", if valid { "成功" } else { "失败" });
                }
                Err(e) => {
                    println!("验证错误: {}", e);
                }
            }
        }
        Err(e) => {
            println!("哈希生成错误: {}", e);
        }
    }
}
