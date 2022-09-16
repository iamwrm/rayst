import rayst
import time


def func_in_py():
    time.sleep(2)


print(rayst.sum_as_string(5, 20))
print(rayst.multiply(5, 20))


rayst.call_2_times(func_in_py)
