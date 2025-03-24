release:
  cargo build -r --target x86_64-unknown-linux-musl
  scp target/x86_64-unknown-linux-musl/release/airvpn-fixer zima:/home/baakel/airfixer
