def nextClosestTime(time):
    """
    :type time: str
    :rtype: str
    """
    hour,minute = time.split(':')

    nums = sorted(set(hour+minute))
    two_digits = [a+b for a in nums for b in nums]

    minute_idx = two_digits.index(minute)
    if minute_idx + 1 < len(two_digits) and two_digits[minute_idx+1] < '60':
        return hour + ':' + two_digits[minute_idx+1]

    hour_idx = two_digits.index(hour)
    if hour_idx + 1 < len(two_digits) and two_digits[hour_idx+1] < '24':
        return two_digits[hour_idx+1] + ':' + two_digits[0]

    return two_digits[0] + ':' + two_digits[0]

print(nextClosestTime("19:34"))