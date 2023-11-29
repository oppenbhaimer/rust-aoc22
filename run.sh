for f in src/bin/*; do
    cargo r --bin $(basename $f .rs);
done;
