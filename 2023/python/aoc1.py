import fileinput
import os
import re

DIGITS = {
    'one': '1',
    'two': '2',
    'three': '3',
    'four': '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9'
}
RE_DIGITS = re.compile('|'. join(DIGITS.keys()))

def debug(str):
    if os.environ.get('DEBUG', None):
        print(str)

def extract_calibration_value(line):
    first = None
    last = None

    for char in line:
        if char.isdigit():
            found = char
            if first is None:
                first = found
            last = found
    
    if first is None or last is None:
        print(f"WARN: Did not find any digits for line: " + line)
        return 0
    else:
        return int(first + last)

def replace_spelled_out(line):
    output = ""
    for idx in range(len(line)):
        if line[idx].isdigit():
            output = output + line[idx]
        m = RE_DIGITS.match(line[idx:])
        if m:
            output = output + DIGITS[m[0]]
    return output

raw_count = 0
processed_count = 0
parsed_count = 0
for line in fileinput.input():
    raw_value = extract_calibration_value(line)
    debug(">> RAW: " + str(raw_value) + " // " + line.strip())
    raw_count = raw_count + raw_value

    processed_line = replace_spelled_out(line)
    processed_value = extract_calibration_value(processed_line)
    debug(">>   P: " + str(processed_value) + " // " + processed_line.strip())
    processed_count = processed_count + processed_value

print("Raw count: " + str(raw_count))
print("Processed count: " + str(processed_count))