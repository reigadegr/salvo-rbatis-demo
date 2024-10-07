# salvo-rbatis-demo
salvo框架的demo
### 编译
#### 基础环境配置
- 下载容器
```shell
pkg install proot -y; pkg install proot-distro -y; proot-distro add archlinux
```

- 登录容器

```shell
proot-distro login archlinux
```

- 更新源

```shell
yes | pacman -Sy
```

- 安装依赖
```shell
yes | pacman -S llvm clang python glibc
```
安装glibc是防止以下问题:
```
= note: cc: /usr/lib/libc.so.6: version `GLIBC_2.36' not found (required by cc)
      cc: /usr/lib/libc.so.6: version `GLIBC_2.38' not found (required by cc)
```
- 安装rust
> 默认为nightly，default
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --profile default -y
```
```shell
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```
```shell
rustup component add rust-src
```
```shell
cargo install cargo-ndk
```

- 下载android NDK
- aarch64架构:

  https://github.com/Lzhiyong/termux-ndk/releases

- x86_64架构：
  https://github.com/android/ndk/releases/latest

- 下载完毕后，把下载好的zip改名为ndk.zip，随后按照以下代码设置
```shell
mkdir ~/ndk_temp 2>/dev/null
unzip ndk.zip -d ~/ndk_temp
mv ~/ndk_temp/*/* ~/ndk_temp
```
- 随后设置环境变量
```shell
export ANDROID_NDK_HOME=$(realpath ~/ndk_temp)
export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
```
全部设置完毕后，执行`sh build.sh release` 即可
