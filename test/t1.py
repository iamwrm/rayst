import rayst
import time


def func_in_py():
    time.sleep(5)
    print("func_in_py")


print(rayst.sum_as_string(5, 20))
print(rayst.multiply(5, 20))

t1 = time.time()
rayst.call_2_times(func_in_py)
print("call_2_times: ", time.time() - t1)
