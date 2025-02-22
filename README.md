- Create app 
cargo new --bin --vcs none --name auth api/auth
- Create lib
cargo new --lib --vcs none --name shared-shared-data-app lib/shared/shared/data/app
- Generate migrate
sea-orm-cli migrate init -d ./features/auth/migration


- Docker
docker-compose -f docker-compose.no_api.yml up -d
Access redis-command http://localhost:8081/
Access PG ADMINADMIN http://localhost:5050/ Account: admin@admin.com/password123


// TODO need implement compiler
https://dev.to/olutolax/building-a-compiler-interpreter-in-rust-part-3-em2

- Install Rust on Linux
login: nghiandd/123456789
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source $"($nu.home-path)/.cargo/env.nu"  # For nushell

Run on linux ubuntu
To have systemd start code-server now and restart on boot:
  sudo systemctl enable --now code-server@$USER
Or, if you don't want/need a background service you can run:
  code-server

  netsh interface portproxy add v4tov4 listenport=8080 listenaddress=0.0.0.0 connectport=8080 connectaddress=$(wsl hostname -i)