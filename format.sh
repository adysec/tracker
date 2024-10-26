#!/bin/bash

# 检查是否提供了文件名作为参数
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <file>"
    exit 1
fi

input_file="$1"
output_file="formatted_trackers.txt"

# 检查文件是否存在
if [ ! -f "$input_file" ]; then
    echo "Error: File not found."
    exit 1
fi

# 使用tr将逗号替换为换行符，然后使用grep和sed来处理文件
# - 使用grep -oP 来匹配以http://, https://, udp://, wss://开头并且以/announce结尾的内容
# - 不进行贪婪匹配
# - 去除每行末尾的所有空格或制表符
tr ',' '\n' < "$input_file" | \
grep -oP '(http|https|udp|wss)://[^/]+/announce' | \
sed 's/[ \t]*$//' > "$output_file"

echo "Formatted trackers have been saved to $output_file"
