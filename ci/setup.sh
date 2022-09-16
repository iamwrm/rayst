sudo rm -rf $HOME/.rustup/
curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup component add clippy
rustup target add x86_64-unknown-linux-musl
cargo install cargo-edit &
wget -q -c https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
bash Miniconda3-latest-Linux-x86_64.sh -b

CONDA_PATH=$HOME/miniconda3
eval "$($CONDA_PATH/bin/conda shell.bash hook)"

wait
