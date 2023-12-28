# The goal is to count the frequency of consecutive strings in a row. 

def count_subsequent_str(str):
    temp = {}
    for i in range(len(str) - 1):
        if not str[i] in temp:
            temp[str[i]] = 0
        if str[i] == str[i+1] and temp[str[i]] < 1:
            temp[str[i]] += 2
        elif str[i] == str[i+1] and temp[str[i]] > 0:
            temp[str[i]] += 1

    return temp

if __name__ == '__main__':
    result = count_subsequent_str("nBalooonnn")
    print(result)

    # {'n': 3, 'B': 0, 'a': 0, 'l': 0, 'o': 3}