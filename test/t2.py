"""
    This file is the prototype of the API of rayst lib
"""


class RaystP:
    def __init__(self) -> None:
        pass

    def run_par(self, func, params_list, num_cpus):
        return_list = []
        for i in params_list:
            return_list.append(func(i))
        return return_list


def func_remote(params):
    return (params[0]*2, params[1]*2)


if __name__ == '__main__':
    params_list = [(1, 2), (2, 4)]

    rayst_p = RaystP()

    return_list = rayst_p.run_par(func_remote, params_list, num_cpus=2)

    for i in return_list:
        print(i)
