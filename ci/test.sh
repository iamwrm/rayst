CONDA_PATH=$HOME/miniconda3
eval "$($CONDA_PATH/bin/conda shell.bash hook)"

pip install maturin

pushd rayst 
maturin develop --features "extension-module"
popd


python test/t1.py

# pushd rayst
# LD_LIBRARY_PATH=$HOME/miniconda3/lib cargo run --package rayst --bin t1
# popd
