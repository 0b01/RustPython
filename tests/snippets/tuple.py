assert (1,2) == (1,2)

x = (1,2)
assert x[0] == 1

y = (1,)
assert y[0] == 1

assert x + y == (1, 2, 1)

assert x * 3 == (1, 2, 1, 2, 1, 2)
# assert 3 * x == (1, 2, 1, 2, 1, 2)
assert x * 0 == ()
assert x * -1 == ()  # integers less than zero treated as 0

assert y < x, "tuple __lt__ failed"
assert x > y, "tuple __gt__ failed"


b = (1,2,3)
assert b.index(2) == 1
