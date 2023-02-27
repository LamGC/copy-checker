# copy-checker

A simple folder replication check tool.  
If you are unable to determine whether the folder was successfully copied, you can use this tool to check.

## Usage

Suppose you have copied a folder to the other side, but for some reason,
you are not sure whether the copy is successful, then you can use this tool to check:

```powershell
./copy-checker /source-dir /target-dir
```

After execution, the tool will automatically check whether the files in the `source-dir` are correctly copied to the `target-dir` and output the results to a csv file (the default is' result. csv 'in the running directory).  

If you want to specify the path of the result file, just do this:  

```powershell
./copy-checker /source-dir /target-dir ./xxx-result.csv
```
