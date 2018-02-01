from ctypes import cdll

lib = cdll.LoadLibrary('target/release/librustfrompy.so')


def test_rust_add():
    assert lib.add(27, 15) == 42
