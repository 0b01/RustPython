def assert_raises(expr, exc_type):
    """
    Helper function to assert `expr` raises an exception of type `exc_type`
    Args:
        expr: Callable
        exec_type: Exception
    Returns:
        None
    Raises:
        Assertion error on failure
    """
    try:
        expr(None)
    except exc_type:
        assert True
    else:
        assert False

assert range(2**63+1)[2**63] == 9223372036854775808

# len tests
assert len(range(10, 5)) == 0, 'Range with no elements should have length = 0'
assert len(range(10, 5, -2)) == 3, 'Expected length 3, for elements: 10, 8, 6'
assert len(range(5, 10, 2)) == 3, 'Expected length 3, for elements: 5, 7, 9'

# index tests
assert range(10).index(6) == 6
assert range(4, 10).index(6) == 2
assert range(4, 10, 2).index(6) == 1
assert range(10, 4, -2).index(8) == 1

# index raises value error on out of bounds
assert_raises(lambda _: range(10).index(-1), ValueError)
assert_raises(lambda _: range(10).index(10), ValueError)

# index raises value error if out of step
assert_raises(lambda _: range(4, 10, 2).index(5), ValueError)

# index raises value error if needle is not an int
assert_raises(lambda _: range(10).index('foo'), ValueError)

# __bool__
assert bool(range(1))
assert bool(range(1, 2))

assert not bool(range(0))
assert not bool(range(1, 1))

# __contains__
assert 6 in range(10)
assert 6 in range(4, 10)
assert 6 in range(4, 10, 2)
assert 10 in range(10, 4, -2)
assert 8 in range(10, 4, -2)

assert -1 not in range(10)
assert 9 not in range(10, 4, -2)
assert 4 not in range(10, 4, -2)
assert 'foo' not in range(10)

# __reversed__
assert list(reversed(range(5))) == [4, 3, 2, 1, 0]
assert list(reversed(range(5, 0, -1))) == [1, 2, 3, 4, 5]
