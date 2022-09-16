CONDA_PATH=$HOME/miniconda3
eval "$($CONDA_PATH/bin/conda shell.bash hook)"

pip install maturin

pushd rayst 
maturin develop
popd


python test/t1.py

