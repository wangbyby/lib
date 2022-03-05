
- AsyncReadExt
    - read: async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
        1.当文件大小小于buf时, 正常读取文件至EOF
        2.当文件已经到EOF时, 返回Ok(0)
        3.当buf长度为0, 返回Ok(0)
    - read_exact
        1. 文件大小小于buf长度,返回Error 
        2. 文件大小≥buf长度,返回Ok(长度)
    - read_buf
      - 写入BytesMut结构体
    - read_to_end: 读取全部
    - read_to_string: