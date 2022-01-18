# minigrep

# 一个I/O项目：命令行程序

实现了通过命令行匹配文档中含有相关文字的行，并打印在output.txt中

命令行参数输入：
  第一个参数为要匹配的文字
  第二个参数为文档名称
  可通过设置CASE_INSENSITIVE=1，让匹配忽略大小写
  
  例：CASE_INSENSITIVE=1 cargo run the poem.txt
