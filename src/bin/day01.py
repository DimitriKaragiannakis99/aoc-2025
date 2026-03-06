import math

def solve(path):
    with open(path, 'r') as f:
        commands = [line.strip() for line in f if line.strip()]
    
    dial = 50
    count = 0

    for command in commands:
        print('Count ', count, ' Dial: ', dial, ' Command: ', command)
        direction = command[0]
        increment = int(command[1:])

        if direction == 'L':
            dial_before = dial
            dial -= increment
            rotations = (-dial_before + increment - 1) // 100 + 1 if dial < 0 else 0
            count += rotations
        else:
            dial_before = dial
            dial += increment
            rotations = (dial_before + increment) // 100
            count += rotations
       

        dial %= 100

    print(count)

if __name__ == "__main__":
    day01_file_path = '../../inputs/day01.txt'
    solve(day01_file_path)
