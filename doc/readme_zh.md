<br>简体中文 | [English](https://github.com/gngpp/ninja/blob/main/doc/readme.md)

如果项目对你有帮助，请考虑[捐赠支持](https://github.com/gngpp/gngpp/blob/main/SPONSOR.md#sponsor-my-open-source-works)项目持续维护，也可以付费获取咨询和技术支持服务。

### 安装

- #### 平台支持

  - `x86_64-unknown-linux-musl`
  - `aarch64-unknown-linux-musl`
  - `armv7-unknown-linux-musleabi`
  - `armv7-unknown-linux-musleabihf`
  - `arm-unknown-linux-musleabi`
  - `arm-unknown-linux-musleabihf`
  - `armv5te-unknown-linux-musleabi`
  - `x86_64-pc-windows-msvc`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`

- #### Ubuntu(Other Linux)

  GitHub [Releases](https://github.com/gngpp/ninja/releases/latest) 中有预编译的 deb包，二进制文件，以Ubuntu为例：

```shell
wget https://github.com/gngpp/ninja/releases/download/v0.8.8/ninja-0.8.8-x86_64-unknown-linux-musl.tar.gz
tar -xf ninja-0.8.8-x86_64-unknown-linux-musl.tar.gz
./ninja run
```

- #### OpenWrt

GitHub [Releases](https://github.com/gngpp/ninja/releases/latest) 中有预编译的 ipk 文件， 目前提供了 aarch64/x86_64 等架构的版本，下载后使用 opkg 安装，以 nanopi r4s 为例：

```shell
wget https://github.com/gngpp/ninja/releases/download/v0.8.8/ninja_0.8.8_aarch64_generic.ipk
wget https://github.com/gngpp/ninja/releases/download/v0.8.8/luci-app-ninja_1.1.6-1_all.ipk
wget https://github.com/gngpp/ninja/releases/download/v0.8.8/luci-i18n-ninja-zh-cn_1.1.6-1_all.ipk

opkg install ninja_0.8.8_aarch64_generic.ipk
opkg install luci-app-ninja_1.1.6-1_all.ipk
opkg install luci-i18n-ninja-zh-cn_1.1.6-1_all.ipk
```

- #### Docker

> 镜像源支持`gngpp/ninja:latest`/`ghcr.io/gngpp/ninja:latest`

```shell
docker run --rm -it -p 7999:7999 --name=ninja \
  -e WORKERS=1 \
  -e LOG=info \
  ghcr.io/gngpp/ninja:latest run
```

- Docker Compose

> `CloudFlare Warp`你的地区不支持（China）请把它删掉，或者你的`VPS`IP可直连`OpenAI`，那么也可以删掉

```yaml
version: '3'

services:
  ninja:
    image: gngpp/ninja:latest
    container_name: ninja
    restart: unless-stopped
    environment:
      - TZ=Asia/Shanghai
      - PROXIES=socks5://warp:10000
    command: run
    ports:
      - "8080:7999"
    depends_on:
      - warp

  warp:
    container_name: warp
    image: ghcr.io/gngpp/warp:latest
    restart: unless-stopped

  watchtower:
    container_name: watchtower
    image: containrrr/watchtower
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    command: --interval 3600 --cleanup
    restart: unless-stopped

```

### ArkoseLabs

发送`GPT-4/GPT-3.5/创建API-Key`对话需要`Arkose Token`作为参数发送，支持的解决方案暂时只有两种

1) 使用HAR

- 支持HAR特征池化，可同时上传多个HAR，使用轮训策略

`ChatGPT` 官网发送一次 `GPT-4` 会话消息，浏览器 `F12` 下载 `https://tcr9i.chat.openai.com/fc/gt2/public_key/35536E1E-65B4-4D96-9D97-6ADB7EFF8147` 接口的HAR日志记录文件，使用启动参数 `--arkose-gpt4-har-dir` 指定HAR目录路径使用（不指定路径则使用默认路径`~/.gpt4`，可直接上传更新HAR），同理`GPT-3.5`和其他类型也是一样方法。支持WebUI上传更新HAR，请求路径:`/har/upload`，可选上传身份验证参数:`--arkose-har-upload-key`

2) 使用[YesCaptcha](https://yescaptcha.com/i/1Cc5i4) / [CapSolver](https://dashboard.capsolver.com/passport/register?inviteCode=y7CtB_a-3X6d)

平台进行验证码解析，启动参数`--arkose-solver`选择平台（默认使用`YesCaptcha`），`--arkose-solver-key` 填写`Client Key`

- 两种方案都使用，优先级是：`HAR` > `YesCaptcha` / `CapSolver`
- `YesCaptcha` / `CapSolver`推荐搭配HAR使用，出验证码则调用解析器处理，验证后HAR使用更持久

目前OpenAI已经更新`登录`需要验证`Arkose Token`，解决方式同`GPT-4`，填写启动参数指定HAR文件`--arkose-auth-har-dir`。创建API-Key需要上传Platform相关的HAR特征文件，获取方式同上。

近日，`OpenAI`取消对`GPT-3.5`进行`Arkose`验证，可以不上传HAR特征文件使用（已上传的不影响），兼容后续可能会再次开启`Arkose`验证，需要加上启动参数`--arkose-gpt3-experiment`进行开启`GPT-3.5`模型`Arkose`验证处理，WebUI不受影响.

### Http 服务

#### 公开接口, `*` 表示任意`URL`后缀

- ChatGPT-API
  - `/public-api/*`
  - `/backend-api/*`
  
- OpenAI-API
  - `/v1/*`

- Platform-API
  - `/dashboard/*`
  
- ChatGPT-To-API
  - `/v1/chat/completions`
  > 关于`ChatGPT`转`API`使用方法，`AceessToken`当`API Key`使用

- Files-API
  - `/files/*`
  > 图片和文件上下传API代理，`/backend-api/files`接口返回的API已经转为`/files/*`

- Authorization
  - 登录: `/auth/token`，表单`option`可选参数，默认为`web`登录，返回`AccessToken`与`Session`；参数为`apple`/`platform`，返回`AccessToken`与`RefreshToken`
  - 刷新 `RefreshToken`: `/auth/refresh_token`
  - 撤销 `RefreshToken`: `/auth/revoke_token`
  - 刷新 `Session`: `/api/auth/session`，发送名为`__Secure-next-auth.session-token`的Cookie调用刷新`Session`，同时返回新的`AccessToken`
  
  `Web登录`默认返回一个名为: `__Secure-next-auth.session-token`的cookie，客户端只需要保存这个cookie，调用`/api/auth/session`也可以刷新`AccessToken`

  `RefreshToken`获取的方式，采用`Apple`平台`ChatGPT App`登录方式，原理是使用内置MITM代理。`Apple设备`连上代理即可开启`Apple平台`登录获取`RefreshToken`，仅适用于量小或者个人使用`（量大会封设备，慎用）`，详细使用请看启动参数说明。

  ```shell
  # 生成证书
  ninja genca

  ninja run --pbind 0.0.0.0:8888

  # 手机设置网络设置你代理监听地址，例如: http://192.168.1.1:8888
  # 之后浏览器打开 http://192.168.1.1:8888/preauth/cert，下载证书安装并信任，之后打开iOS ChatGPT就可以愉快玩耍了
  ```

#### API文档

- Platfrom API [doc](https://platform.openai.com/docs/api-reference)
- Backend API [doc](https://github.com/gngpp/ninja/blob/main/doc/rest.http)

#### 基本服务

- ChatGPT WebUI
- 公开`ChatGPT-API`/`OpenAI-API`代理
- `API`前缀与官方一致
- `ChatGPT` 转 `API`
- 可接入第三方客户端
- 可接入IP代理池，提高并发
- 支持获取RefreshToken
- 支持以HAR格式文件特征池

#### 参数说明

- `--level`，环境变量 `LOG`，日志级别: 默认info
- `--bind`，环境变量 `BIND`， 服务监听地址: 默认0.0.0.0:7999，
- `--tls-cert`，环境变量 `TLS_CERT`，TLS证书公钥，支持格式: EC/PKCS8/RSA
- `--tls-key`，环境变量 `TLS_KEY`，TLS证书私钥
- `--workers`，工作线程: 默认1
- `--disable-webui`, 如果不想使用默认自带的WebUI，使用此参数关闭
- `--enable-file-proxy`，环境变量`ENABLE_FILE_PROXY`，开启文件上下传API代理
- `--enable-direct`，开启直连，将绑定`interface`出口的IP的加入代理池
- `--proxies`，代理，支持代理池，多个代理使用`,`隔开，格式: protocol://user:pass@ip:port

##### 代理高阶用法

分代理内置协议和代理类型，内置协议: `all/api/auth/arkose`，其中`all`针对所有客户端，`api`针对所有`OpenAI API`，`auth`针对授权/登录，`arkose`针对ArkoseLabs；代理类型: `interface/proxy/ipv6_subnet`，其中`interface`表示绑定的出口`IP`地址，`proxy`表示上游代理协议: `http/https/socks5`，`ipv6_subnet`表示用Ipv6子网段内随机IP地址作为代理。格式为`proto|proxy`，例子: **`all|socks5://192.168.1.1:1080, api|10.0.0.1, auth|2001:db8::/32, http://192.168.1.1:1081`**，不带内置协议，协议默认为`all`。
  
##### 代理使用规则

1) 存在`interface` \ `proxy` \ `ipv6_subnet`

当开启`--enable-direct`，那么将使用`proxy` + `interface`作为代理池；未开启`--enable-direct`，只有`proxy`数量大于等于2才使用`proxy`，否则将使用 `ipv6_subnet`作为代理池，`interface`作为fallback地址。

2) 存在`interface`、`proxy`

当开启`--enable-direct`，那么将使用`proxy` + `interface`作为代理池；未开启`--enable-direct`，只使用`proxy`作为代理池。
  
3) 存在`proxy` \ `ipv6_subnet`

规则同(1)，只是没有`interface`作为fallback地址。

4) 存在`interface` \ `ipv6_subnet`
当开启`--enable-direct`，同时`interface`数量大于等于2似，`interface`作为代理池；未开启`--enable-direct`，将使用 `ipv6_subnet`作为代理池，`interface`作为fallback地址。

5) 存在`proxy`

当开启`--enable-direct`,使用`proxy` + 默认直连作为代理池；未开启`--enable-direct`，只使用`proxy`作为代理池

6) 存在`ipv6_subnet`

无论是否开启`--enable-direct`，都将使用`ipv6_subnet`作为代理池

### 命令手册

```shell
$ ninja --help
Reverse engineered ChatGPT proxy

Usage: ninja [COMMAND]

Commands:
  run      Run the HTTP server
  stop     Stop the HTTP server daemon
  start    Start the HTTP server daemon
  restart  Restart the HTTP server daemon
  status   Status of the Http server daemon process
  log      Show the Http server daemon log
  genca    Generate MITM CA certificate
  gt       Generate config template file (toml format file)
  update   Update the application
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ ninja run --help
Run the HTTP server

Usage: ninja run [OPTIONS]

Options:
  -L, --level <LEVEL>
          Log level (info/debug/warn/trace/error) [env: LOG=] [default: info]
  -C, --config <CONFIG>
          Configuration file path (toml format file) [env: CONFIG=]
  -b, --bind <BIND>
          Server bind address [env: BIND=] [default: 0.0.0.0:7999]
  -W, --workers <WORKERS>
          Server worker-pool size (Recommended number of CPU cores) [default: 1]
      --concurrent-limit <CONCURRENT_LIMIT>
          Enforces a limit on the concurrent number of requests the underlying [default: 1024]
      --enable-direct
          Enable direct connection [env: ENABLE_DIRECT=]
  -x, --proxies <PROXIES>
          Request client proxy, support multiple proxy, use ',' to separate
          Format: proto|type
          Proto: all/api/auth/arkose, default: all
          Type: interface/proxy/ipv6 subnet，proxy type only support: socks5/http/https
          Example: all|socks5://192.168.1.1:1080, api|10.0.0.1, auth|2001:db8::/32, http://192.168.1.1:1081 [env: PROXIES=]
      --cookie-store
          Enabled Cookie Store [env: COOKIE_STORE=]
      --timeout <TIMEOUT>
          Client timeout (seconds) [default: 360]
      --connect-timeout <CONNECT_TIMEOUT>
          Client connect timeout (seconds) [default: 20]
      --tcp-keepalive <TCP_KEEPALIVE>
          TCP keepalive (seconds) [default: 60]
      --pool-idle-timeout <POOL_IDLE_TIMEOUT>
          Set an optional timeout for idle sockets being kept-alive [default: 90]
      --tls-cert <TLS_CERT>
          TLS certificate file path [env: TLS_CERT=]
      --tls-key <TLS_KEY>
          TLS private key file path (EC/PKCS8/RSA) [env: TLS_KEY=]
      --cf-site-key <CF_SITE_KEY>
          Cloudflare turnstile captcha site key [env: CF_SECRET_KEY=]
      --cf-secret-key <CF_SECRET_KEY>
          Cloudflare turnstile captcha secret key [env: CF_SITE_KEY=]
  -A, --auth-key <AUTH_KEY>
          Login Authentication Key [env: AUTH_KEY=]
  -D, --disable-webui
          Disable WebUI [env: DISABLE_WEBUI=]
  -F, --enable-file-proxy
          Enable file proxy [env: ENABLE_FILE_PROXY=]
      --arkose-endpoint <ARKOSE_ENDPOINT>
          Arkose endpoint, Example: https://client-api.arkoselabs.com
  -E, --arkose-gpt3-experiment
          Enable Arkose GPT-3.5 experiment
      --arkose-gpt3-har-dir <ARKOSE_GPT3_HAR_DIR>
          About the browser HAR directory path requested by ChatGPT GPT-3.5 ArkoseLabs
      --arkose-gpt4-har-dir <ARKOSE_GPT4_HAR_DIR>
          About the browser HAR directory path requested by ChatGPT GPT-4 ArkoseLabs
      --arkose-auth-har-dir <ARKOSE_AUTH_HAR_DIR>
          About the browser HAR directory path requested by Auth ArkoseLabs
      --arkose-platform-har-dir <ARKOSE_PLATFORM_HAR_DIR>
          About the browser HAR directory path requested by Platform ArkoseLabs
  -K, --arkose-har-upload-key <ARKOSE_HAR_UPLOAD_KEY>
          HAR file upload authenticate key
  -s, --arkose-solver <ARKOSE_SOLVER>
          About ArkoseLabs solver platform [default: yescaptcha]
  -k, --arkose-solver-key <ARKOSE_SOLVER_KEY>
          About the solver client key by ArkoseLabs
  -T, --tb-enable
          Enable token bucket flow limitation
      --tb-store-strategy <TB_STORE_STRATEGY>
          Token bucket store strategy (mem/redis) [default: mem]
      --tb-redis-url <TB_REDIS_URL>
          Token bucket redis connection url [default: redis://127.0.0.1:6379]
      --tb-capacity <TB_CAPACITY>
          Token bucket capacity [default: 60]
      --tb-fill-rate <TB_FILL_RATE>
          Token bucket fill rate [default: 1]
      --tb-expired <TB_EXPIRED>
          Token bucket expired (seconds) [default: 86400]
  -B, --pbind <PBIND>
          Preauth MITM server bind address [env: PREAUTH_BIND=]
  -X, --pupstream <PUPSTREAM>
          Preauth MITM server upstream proxy, Only support http/https/socks5 protocol [env: PREAUTH_UPSTREAM=]
      --pcert <PCERT>
          Preauth MITM server CA certificate file path [default: ca/cert.crt]
      --pkey <PKEY>
          Preauth MITM server CA private key file path [default: ca/key.pem]
  -h, --help
          Print help
```

### 编译

- Linux编译，Ubuntu机器为例:

```shell
apt install build-essential
apt install cmake
apt install libclang-dev

git clone https://github.com/gngpp/ninja.git && cd ninja
cargo build --release
```

- OpenWrt 编译

```shell
cd package
svn co https://github.com/gngpp/ninja/trunk/openwrt
cd -
make menuconfig # choose LUCI->Applications->luci-app-ninja  
make V=s
```
