# copy-checker

一个简单的文件夹复制检查工具。  
如果你无法确定文件夹是否复制成功，那么就可以用这个工具进行检查。  

## 使用

假设你现在把某个文件夹复制到了另一边，但由于某些原因，你不确定是否复制成功，那么你可以使用这个工具进行检查。

```powershell
./copy-checker /source-dir /target-dir
```

执行后，工具会自动检查 `source-dir` 中的文件是否正确复制到了 `target-dir` 中，并将结果输出到一个 csv 文件中（默认在运行目录中的 `result.csv`）。  

如果你想要指定结果文件的路径，只需要这样：

```powershell
./copy-checker /source-dir /target-dir ./xxx-result.csv
```
