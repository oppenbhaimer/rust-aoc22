cookie="session=<Replace with your AoC session cookie>"

for i in {1..25}; do
    curl -v --cookie $cookie https://adventofcode.com/2022/day/$i/input -o input/input_$i;
done
