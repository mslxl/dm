计划用中文写了，英语描述不清。

## 功能(与卫星)

### 本地管理
DM 并不打算直接管理文件，而是管理 group，每个文件必须有一个 group，而且
同步配置文件时以 group 为单位，不能只同步单个文件

#### 新建 group
```bash
dm new gruop <group-name>
# dm n g <group-name>
```

#### 添加文件到 group
```bash
dm add [file-path...] <group-name>
# dm a [file-path...] <group-name>
```
不提供 group-name 直接报错

group-name 中不允许出现路径名中不允许出现的特殊字符（方便管理）

#### 从 group 中删除文件
```bash
dm rm <file-path>
```
如果文件不属于任何 group，则直接报错

#### 更新文件
```bash
dm update [group-name...]
```
不提供 group-name 则更新全部 group

#### 安装文件
```bash
dm install [group-name...]
```
不提供 group-name 则拒绝运行

如果需要安装所有符合条件的 group，添加参数 `-a`
