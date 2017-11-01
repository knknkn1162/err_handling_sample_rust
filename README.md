# Error handing example 

Rust has Option and Result traits and has no exception, so the way of error handling is different from other popular languages, like golang or Python.

Some may be confused if he use it for the first time, so do I.. Here is an common example of handling error in rust.

## Problem setting

Read file, parse numbers as i32 and aggregate them.  Multiple Errors could occur when reading file or parsing numbers from String. 

```
input: file
output: i32
```



## references

see also, 

+ https://rustbyexample.com/error/multiple_error_types/wrap_error.html

+ https://qiita.com/termoshtt/items/8c015d9289613ec640f1
