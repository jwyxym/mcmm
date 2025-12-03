<h1>Environment</h1>

<img src="https://avatars.githubusercontent.com/u/5430905?s=200&v=4"/>
<a href="https://rust-lang.org/">Install Rust</a>

<h1>How To Build</h1>

```bash
git clone https://github.com/jwyxym/mcmm.git --depth 1
cd mcmm
cargo build --release
```

<h1>How To Use</h1>

```bash
+------------+-----------+----------------------+
| options    | functions | parameters           |
+------------+-----------+----------------------+
| init       | 初始化    |                      |
+------------+-----------+----------------------+
| install, i | 下载      |                      |
+------------+-----------+----------------------+
| add, a     | 添加      | id(来自modrinth.com) |
+------------+-----------+----------------------+
| search, s  | 搜索      | 关键词               |
+------------+-----------+----------------------+
| clear, c   | 清空      |                      |
+------------+-----------+----------------------+
```

<h1>Quick Start</h1>

```bash
mcmm init 1.21.1 neoforge
# add new mod by modrinth id
mcmm add {id} #copy from modrinth.com
# search mods
mcmm search {keyword}
```