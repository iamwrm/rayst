sudo rm -rf $HOME/.rustup/
curl https://sh.rustup.rs -sSf | sh -s -- -y
wget -q -c https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
bash Miniconda3-latest-Linux-x86_64.sh -b
rustup component add clippy
rustup target add x86_64-unknown-linux-musl
cargo install cargo-edit
