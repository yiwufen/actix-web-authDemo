"Before starting, please use `cargo test` to create the relevant tables."

> ```rust
> #[cfg(test)]
> mod usertable {
>     use super::*;
>     #[test]
>     fn test_create_user_table() {
>         create_user_table();
>     }
> }
> ```

**run**

```
cargo test
cargo run
```

### Configure OpenSSL

#### Generating Trusted Certificate

We put self-signed certificate in this directory as an example but your browser will complain that connections to the server aren't secure. We recommend to use [`mkcert`](https://github.com/FiloSottile/mkcert) to trust it. To use a local CA, you should run:

```
mkcert -install
```

If you want to generate your own private key/certificate pair, then run:

```
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

A new `key.pem` and `cert.pem` will be saved to the current directory. You will then need to modify `main.rs` where indicated.

#### install openssl [windows]

1. clone [vcpkg](https://github.com/Microsoft/vcpkg)

2. open directory where you've cloned vcpkg

3. run `./bootstrap-vcpkg.bat`

4. run `./vcpkg.exe install openssl-windows:x64-windows`

5. run `./vcpkg.exe install openssl:x64-windows-static`

6. run `./vcpkg.exe integrate install`

7. run `set VCPKGRS_DYNAMIC=1` (or simply set it as your environment variable)

8. Add environment variables`$env:OPENSSL_DIR="<vcpkg>\installed\x64-windows-static"`

9. Ensure that the environment variables already exist in the terminal:powershell run`$env:OPENSSL_DIR`

   ```
   PS D:\...\actix-web-authDemo> $env:OPENSSL_DIR
   D:\installed\vcpkg\installed\x64-windows-static
   ```
#### install openssl [linux]
1. Install `pkg-config`: You need to install `pkg-config` on your Linux distribution. Depending on your operating system, the installation command might vary:
For Ubuntu/Debian: `sudo apt install pkg-config`
For Fedora: `sudo dnf install pkg-config`
For CentOS/RedHat: `sudo yum install pkg-config`
For Alpine Linux: `sudo apk add pkgconfig`

2. Install OpenSSL Development Package: You also need to make sure that the development package of `OpenSSL` is installed:
For Ubuntu/Debian: `sudo apt install libssl-dev`
For Fedora: `sudo dnf install openssl-devel`
For CentOS/RedHat: `sudo yum install openssl-devel`
For Alpine Linux: `sudo apk add openssl-dev`

4. Set Environment Variable (if necessary): If you still encounter issues after installing the above packages, you might need to set the OPENSSL_DIR environment variable to help the openssl-sys crate locate the OpenSSL installation.

   update main.js

   ```rust
   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
       // configure openssl
       let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
       builder
           .set_private_key_file("cert/key.pem", SslFiletype::PEM)
           .unwrap();
       builder.set_certificate_chain_file("cert/cert.pem").unwrap();
       env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
       HttpServer::new(|| {
           App::new()
               ...
               ...
       })
       .bind_openssl("127.0.0.1:8080",builder)?
       // .bind("127.0.0.1:8080")?
       .workers(1)
       .run()
       .await
   }
   
   ```

   ```
   cargo run
   ```

   GET https://127.0.0.1:8080/hello/world
