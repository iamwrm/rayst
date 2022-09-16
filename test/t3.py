from concurrent.futures import ThreadPoolExecutor
import time


def wait_on_a(a):
    for i in range(2000000):
        a = a+1
    return a


def excutor_run(max_workers):
    executor = ThreadPoolExecutor(max_workers=max_workers)

    t1 = time.time()

    promise_list = []
    for i in range(10):
        p = executor.submit(wait_on_a, i)
        promise_list.append(p)

    result_list = []
    for i in promise_list:
        result_list.append(i.result())

    print(f"{max_workers}")
    print("result_list sum: ", sum(result_list))
    print(time.time() - t1)


excutor_run(max_workers=2)
excutor_run(max_workers=4)
excutor_run(max_workers=8)
