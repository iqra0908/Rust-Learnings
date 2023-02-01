def reverse(x: int) -> int:
    if x >= pow(2,31)-1 or x < -pow(2,31):
        return 0

    y = str(x)
    signed = False
    if y[0] == '-':
        y = y[1:]
        signed = True
    z = ""
    if signed:
        z = '-'
    for i in range(len(y)-1,-1,-1):
        z+=y[i]
    return int(z)

print(reverse(-123))