#!/bin/bash

# 检查是否提供了文件名作为参数
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <file>"
    exit 1
fi

input_file="$1"
output_file_main="trackers_best.txt"
output_file_http="trackers_best_http.txt"
output_file_https="trackers_best_https.txt"
output_file_udp="trackers_best_udp.txt"
output_file_wss="trackers_best_wss.txt"

# 检查文件是否存在
if [ ! -f "$input_file" ]; then
    echo "Error: File not found."
    exit 1
fi

# 清空所有输出文件
> "$output_file_main"
> "$output_file_http"
> "$output_file_https"
> "$output_file_udp"
> "$output_file_wss"

# 过滤掉包含blackstr.txt中恶意IP的URL，然后逐行处理
{
    if [ -f "blackstr.txt" ]; then
        grep -v -F -f blackstr.txt "$input_file"
    else
        cat "$input_file"
    fi
} | while IFS= read -r tracker; do
    protocol=$(echo "$tracker" | grep -oE '^[a-z]+')
    
    case $protocol in
        http)
            if curl -s -f -m 1 "$tracker" &>/dev/null; then
                echo "Success: $tracker"
                echo "$tracker" >> "$output_file_main"
                echo "$tracker" >> "$output_file_http"
            else
                echo "Failed: $tracker"
            fi
            ;;
        https)
            if curl -s -f -m 1 "$tracker" &>/dev/null; then
                echo "Success: $tracker"
                echo "$tracker" >> "$output_file_main"
                echo "$tracker" >> "$output_file_https"
            else
                echo "Failed: $tracker"
            fi
            ;;
        udp)
            host=$(echo "$tracker" | cut -d'/' -f3)
            port=$(echo "$host" | cut -d':' -f2)
            host=$(echo "$host" | cut -d':' -f1)
            if nc -zuv -w 1 "$host" "$port" &>/dev/null; then
                echo "Success: $tracker"
                echo "$tracker" >> "$output_file_main"
                echo "$tracker" >> "$output_file_udp"
            else
                echo "Failed: $tracker"
            fi
            ;;
        wss)
            # WSS tracker 测试 - 使用简单的 TCP 连接测试
            host=$(echo "$tracker" | sed 's|wss://||' | cut -d'/' -f1)
            port=$(echo "$host" | cut -d':' -f2)
            if [ "$port" = "$host" ]; then
                port=443  # WSS 默认端口
                host=$(echo "$host" | cut -d':' -f1)
            else
                host=$(echo "$host" | cut -d':' -f1)
            fi
            
            if nc -zv -w 1 "$host" "$port" &>/dev/null; then
                echo "Success: $tracker"
                echo "$tracker" >> "$output_file_main"
                echo "$tracker" >> "$output_file_wss"
            else
                echo "Failed: $tracker"
            fi
            ;;
        *)
            echo "Unknown protocol: $protocol"
            ;;
    esac
done

echo "Testing complete."
