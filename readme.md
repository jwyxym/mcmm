<h1>how to build</h1>
```bash
git clone https://github.com/jwyxym/mcmm.git --depth 1
cd mcmm
cargo build --release
```
<h1>how to use</h1>
```bash
+------------+-----------+
| options    | functions |
+------------+-----------+
| init       | 初始化    |
+------------+-----------+
| install, i | 下载      |
+------------+-----------+
| add, a     | 添加      |
+------------+-----------+
| search, s  | 搜索      |
+------------+-----------+
| clear, c   | 清空      |
+------------+-----------+
```
<h1>quick start</h1>
```bash
mcmm init 1.21.1 neoforge
# add new mod by modrinth id
mcmm add {id} #copy from modrinth.com
# search mods
mcmm search {keyword}
```