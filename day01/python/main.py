with open("../input") as f:
    txt = f.read().splitlines()


num_map = {
    "one": "one1one",
    "two": "two2two",
    "three": "three3three",
    "four": "four4four",
    "five": "five5five",
    "six": "six6six",
    "seven": "seven7seven",
    "eight": "eight8eight",
    "nine": "nine9nine",
}


s = 0
for line in txt:
    for k, v in num_map.items():
        if k in line:
            line = line.replace(k, v)
    nums = [c for c in line if c.isnumeric()]
    num = int(nums[0] + nums[-1])
    print(num)
    s += num

print(s)
