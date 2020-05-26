

# wsl文件权限问题

1. 挂载文件权限

修改/创建 ``/etc/wsl.conf``

```conf
[automount]
enabled = true
root = /mnt/
options = "metadata,dmask=022,fmask=133"
mountFsTab = false

```
将/mnt下的所有盘都挂载为linux下默认的权限

2. wsl创建文件权限

``/etc/profile``或``~/.profile``或``~/.bashrc``添加

```bash
if [[ "$(umask)" == '000' ]]; then
    umask 022
fi
```

3. vscode Remote-wsl插件创建目录权限

```bash
echo umask 022 >> ~/.vscode-server/server-env-setup
```