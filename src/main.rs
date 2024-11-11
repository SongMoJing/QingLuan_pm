mod _lib;

use std::collections::LinkedList;
use clap::{Arg, ArgMatches, Command};
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

	let mut args = get_args().into_iter();

	println!("path:   {}", args.next().unwrap());
	println!("action: {}", args.next().unwrap());
	while let Some(arg) = args.next() {
		println!("arg:    {}", arg);
	}

	exit(0)
}

/// ## 退出程序
/// 提示用户按任意键继续
fn exit(exit_code: i32) {
	base::system("please");
	std::process::exit(exit_code);
}

/// ## 获取命令行参数
/// 读入操作和必要参数
/// 0. 路径
/// 1. 操作
/// 2. 参数...
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
					.default_value(VERSION)))
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
					.default_value("Latest")
					.required(false)))
		.subcommand(Command::new("uninstall")
			.about("Uninstall dependencies")
			.arg(
				Arg::new("packageName")
					.value_name("PACKAGE NAME")
					.help("Specify the name of the package to be uninstall")
					.required(true)))
		.subcommand(Command::new("update")
			.about("Update the dependencies of the 青鸾 project in the directory")
			.arg(
				Arg::new("packageName")
					.value_name("PACKAGE NAME")
					.help("Specify the name of the package to be updated")
					.required(false))
			.arg(
				Arg::new("version")
					.value_name("VERSION")
					.help("Specify the version to which you want to update the package")
					.default_value("Latest")
					.required(false)))
		.arg(Arg::new("path")
			.short('p')
			.long("path")
			.value_name("PATH")
			.help("Object path")
			.default_value("."))
		.get_matches();

	// 获取路径
	let mut path = matches.get_one::<String>("path").unwrap().to_string();
	path = match path.eq(".") {
		true => std::env::current_dir().unwrap().to_str().unwrap().to_string().replace("\\", "/"),
		_ => path.replace("\\", "/"),
	};
	res.push_back(path.to_string());

	// 获取子命令
	if let Some((command, sub_matches)) = matches.subcommand() {
		// 将子命令的名称存入列表
		res.push_back(command.to_string());

		// 根据不同的子命令添加相应的参数
		fn append_cmd(sub_matches: &ArgMatches, res: &mut LinkedList<String>, token: &[&str]) {
			let mut iter = token.iter();
			while let Some(arg) = iter.next() {
				if let Some(v) = sub_matches.get_one::<String>(arg) {
					res.push_back(v.to_string());
				}
			}
		}

		match command {
			"init" => append_cmd(sub_matches, &mut res, &["packageName", "edition", "version"]),
			"remove" => {}
			"install" => append_cmd(sub_matches, &mut res, &["packageName", "version"]),
			"uninstall" => append_cmd(sub_matches, &mut res, &["packageName"]),
			"update" => append_cmd(sub_matches, &mut res, &["packageName", "version"]),
			_ => {}
		}
	}
	res
}
