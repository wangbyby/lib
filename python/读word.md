
1. 安装
```bash
pip install python-docx
```
2. 

3. doc转为docx文件
```python
from win32com import client

import os

def doc_to_docx(path):
    if os.path.splitext(path)[1] == ".doc":
        word = client.Dispatch('Word.Application')
        doc = word.Documents.Open(path)  # 目标路径下的文件
        doc.SaveAs(os.path.splitext(path)[0]+".docx", 16)  # 转化后路径下的文件
        doc.Close()
        word.Quit()

```

4. 读取word(只是docx)文件

    ```python
    import os
    from docx import Document

    def read_docx_document(source_path, save_path: str):
        document = Document(source_path)

        with open(save_path, "a+", encoding="utf-8") as f:
            for p in document.paragraphs:
                f.write(p.text)
        with open(save_path, "a+", encoding="utf-8") as f:
            for t in document.tables:
                for row in t.rows:
                    for cell in row.cells:
                        f.write(cell.text)
    def read_dir(dir_path, save_path):
        print(os.path.abspath(dir_path))
        for i in os.listdir(dir_path):
            get_path = dir_path+i
            # print(get_path)
            if os.path.isdir(get_path) is not True and os.path.splitext(get_path)[1] == ".docx":
                read_docx_document(get_path, save_path)

    ```
