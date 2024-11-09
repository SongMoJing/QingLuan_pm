use std::io;
use std::io::Write;

/// ## 接受用户输入内容
/// 返回输入内容
/// ## Example
/// ```rust
/// let mut input = String::new();
/// input("请输入内容：", &mut input);
/// // input: hello
/// assert_eq!(input, "hello");
/// ```
pub(crate) fn input(tip: &str, res: &mut String) {
    print!("{}", tip);
    io::stdout().flush().expect("Err: 刷新失败");
    io::stdin()
        .read_line(res)
        .expect("Err: 读取输入失败");
}

/// ## 接受用户确认
/// 返回确认结果，非'y'即为否定
/// ## Example
/// ```rust
/// let mut res = false;
/// identify("是否继续？(y/n): ", &mut res);
/// // input: y
/// assert_eq!(res, true);
/// ```
pub(crate) fn identify(tip: &str, res: &mut bool) {
    let mut _input = String::new();
    input(tip, &mut _input);
    *res = _input.trim() == "y";
}

/// ## 按任意键继续
pub(crate) fn please() {
    print!("按任意键继续...");
    io::stdout().flush().expect("Err: 刷新失败");
    io::stdin().read_line(&mut String::new()).expect("Err: 读取输入失败");
}