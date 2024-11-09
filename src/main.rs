mod _lib;

use std::collections::LinkedList;
use clap::{Arg, Command, Parser};
use clap::builder::TypedValueParser;
use colored::*;
use crate::_lib::base;

const VERSION: &str = "S.0.1:2024";
const NAME: &str = "\"青鸾\" Universal Package Manager.";
const AUTHOR: &str = "PRC.松蓦箐 <Song_Mojing@outlook.com>";

/// ## 启用 ANSI 支持
fn enable_ansi_support() {
	let _ = control::set_virtual_terminal(true);
}

fn main() {
	#[cfg(windows)]
	enable_ansi_support();

	let mut args = get_args();
	let mut iter = args.iter();
	println!("{} || {} .", iter.next().unwrap(), iter.next().unwrap());

	// println!("请确认一下信息：\n项目路径: {}\n输出路径: {}\nSDK版本: {}", args[0].green().underline(), args[1].green().underline(), args[2].green().underline());
	// let mut keep = false;
	// base::identify("是否继续？(y/n): ", &mut keep);
	// if keep {
	//     println!("开始编译...");
	// } else {
	//     print!("已取消编译。");
	//     exit(0);
	// }
}

/// ## 退出程序
/// 提示用户按任意键继续
fn exit(exit_code: i32) {
	base::please();
	std::process::exit(exit_code);
}

/// ## 获取命令行参数
/// 读入操作和必要参数
fn get_args() -> LinkedList<String> {
	let mut res: LinkedList<String> = LinkedList::new();
	let matches = Command::new(NAME)
		.version(VERSION)
		.author(AUTHOR)
		.arg_required_else_help(true)
		.subcommand(Command::new("init")
			.about("Create and initialize the 青鸾 project")
			.arg(
				Arg::new("projectName")
					.value_name("PROJECT NAME")
					.help("Specify the name of the project to be created")
					.required(true))
			.arg(
				Arg::new("edition")
					.short('e')
					.long("edition")
					.value_name("VERSION")
					.help("Specify the build criteria")
					.default_value(VERSION))
			.arg(
				Arg::new("version")
					.short('v')
					.long("version")
					.value_name("VERSION")
					.help("Specify the version of the project")
					.default_value("0.1.0")))
		.subcommand(Command::new("remove")
		.about("Remove 青鸾 items from the directory"))
		.subcommand(Command::new("install")
			.about("Install dependencies")
			.arg(
				Arg::new("packageName")
					.value_name("PACKAGE NAME")
					.help("Specify the name of the package to be installed")
					.required(true))
			.arg(
				Arg::new("version")
					.value_name("VERSION")
					.help("Specify the version of the package to be installed")
					.required(false)))
		.subcommand(Command::new("uninstall")
			.about("Uninstall dependencies"))
		.subcommand(Command::new("update")
			.about("Update the dependencies of the Qingluan project in the directory"))
		.arg(Arg::new("path")
			.short('p')
			.long("path")
			.value_name("PATH")
			.help("Object path")
			.default_value("."))
		.get_matches();

	// if let Some((command)) = matches.subcommand() {
	//     res.push_back(command.to_string());
	// }
	//
	// // 获取路径
	// let mut path = matches.get_one::<String>("path").unwrap().to_string();
	// path = if path.eq("The current directory") {
	//     std::env::current_dir().unwrap().to_str().unwrap().to_string()
	// } else {
	//     path.to_string()
	// };
	// res.push_back(path.to_string());
	return res;
}
