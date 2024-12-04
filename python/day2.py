import numpy as np

with open("inputs/day2.txt") as f:
    lines = f.readlines()

reports = [line.split() for line in lines]

# Part 1

def is_safe(report):
    report_diffs = np.ediff1d(np.array(report, dtype=int))
    print(f"report diffs: {report_diffs}")
    if 0 in report_diffs:
        return False
    elif any(report_diffs > 0) and any(report_diffs < 0):
        return False
    elif any(abs(report_diffs) > 3):
        return False
    else:
        return True

safe_count = 0
unsafe_count = 0
for report in reports:
    print(report)
    if is_safe(report):
        print("safe!")
        safe_count += 1
    else:
        print("unsafe!")
        unsafe_count += 1


# Part 2

# def get_unsafe_index(report):
#     report_diffs = np.ediff1d(np.array(report, dtype=int))
#     first_sign = report_diffs[0] / abs(report_diffs[0])
#     index = np.where(report_diffs == 0)
#     if len(index[0]) != 0:
#         return index[0][0] + 1
#     index = np.where(abs(report_diffs) > 3)
#     if len(index[0]) != 0:
#         return index[0][0] + 1
#     index = np.where(report_diffs * first_sign < 0)
#     if len(index[0]) != 0:
#         return index[0][0] + 1
        
def can_be_made_safe(report):
    report = np.array(report, dtype=int)
    for i in range(len(report)):
        changed_report = np.delete(report, i)
        if is_safe(changed_report):
            return True
    return False
    
    

safe_count = 0
unsafe_count = 0
for report in reports:
    print(report)
    if is_safe(report):
        safe_count += 1
    else:
        print("unsafe, trying to make safe")
        if can_be_made_safe(report):
            print("can be made safe!")
            safe_count += 1
        else:
            print("cannot be made safe!")
            unsafe_count += 1