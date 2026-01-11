ROOT_DIR=$(dirname "$0")/..
TOP_NAME="earlgrey"

mkdir -p $ROOT_DIR/build/rdl
echo "Evaluating rdl..."
uv run rdl2ot export-rtl --soc $ROOT_DIR/{rdl/$TOP_NAME.rdl,build/rdl} 

echo "Generating linker script..."
uv run ./$ROOT_DIR/tools/rdlgenerator.py gen-linker-script $ROOT_DIR/{build/rdl/rdl.json,lib/linker/memory.ld}

echo "Generating Rust mmio abstraction..."
mkdir -p $ROOT_DIR/lib/zermio/$TOP_NAME
zermio-cli import-rdl-json --rdl $ROOT_DIR/build/rdl/rdl.json export-rust --dir $ROOT_DIR/lib/zermio/$TOP_NAME
echo "Finished!"

