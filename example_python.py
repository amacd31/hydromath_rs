import numpy as np

from ctypes.util import find_library
from cffi import FFI

__ffi = FFI()
with open('hydromath.h') as header:
    __ffi.cdef(header.read())

lib_name = find_library('hydromath')
__lib = __ffi.dlopen('target/debug/{0}'.format(lib_name))

mse_c = __lib.mse

def mse(obs, sim):
    assert len(obs) == len(sim)

    return mse_c(
        __ffi.cast('double *', obs.ctypes.data),
        __ffi.cast('double *', sim.ctypes.data),
        len(obs)
    )


test_obs = np.array([13.,17.,18.,20.,24.])
test_sim = np.array([12.,15.,20.,22.,24.])

result = mse(test_obs, test_sim)

assert result == 2.6
print("assert {0} == {1}".format(result, 2.6))

result = mse(
    np.array([1.,2.,3.,4.,5.]),
    np.array([1.,2.,3.,4.,5.])
)

assert result == 0
print("assert {0} == {1}".format(result, 0.0))

m = np.mean([1.,2.,3.,4.,5.])
sim = np.array([m, m, m, m, m])
result = mse(np.array([1.,2.,3.,4.,5.]),
            sim
        )

assert result == 2
print("assert {0} == {1}".format(result, 2.0))
