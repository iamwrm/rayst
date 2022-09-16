from concurrent.futures import ThreadPoolExecutor
import time


def wait_on_a(a):
    time.sleep(1)
    a = a+1
    return a


executor = ThreadPoolExecutor(max_workers=19)

t1 = time.time()

promise_list = []
for i in range(20):
    p = executor.submit(wait_on_a, i)
    promise_list.append(p)

result_list = []
for i in promise_list:
    result_list.append(i.result())

print("result_list sum: ", sum(result_list))
print(time.time() - t1)
