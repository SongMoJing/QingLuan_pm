## 包管理器 [pm]

### 参数

`pm <Options> <Command>`

#### -p

> 对象路径

`-p <Path[def: 当前位置]>`

#### init

> 初始化项目

`pm ... init <Project Name> -e <Version>`

| `<Project Name>` : 项目名称

| `-e <Version[def: 包管理器版本]>` : 框架版本（编译）

#### remove

> 移除项目

`pm ... remove`

#### install

> 安装包

`pm ... install <Package Name> <Version>`

| `<Package Name>` : 包名称

| `* <Version[def: Latest]>` : 指定包版本，默认最新

#### uninstall

> 卸载包

`pm ... uninstall <Package Name>`

| `Package Name>` : 包名称

#### update

> 更新包

`pm ... update <Package Name> <Version>`

| `<Package Name>` : 包名称

| `* <Version[def: Latest]>` : 指定包版本，默认最新
