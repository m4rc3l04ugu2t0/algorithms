def factorialofNumber n
  factorial = 1
  count = 2

  while count <= n
    factorial = count * factorial
    count += 1
  end

  factorial
end

# puts factorialofNumber 0

def factorialofNumber2 n
  if n == 0
    return 1
  else
    return n * factorialofNumber2(n - 1)
  end
end

# puts factorialofNumber2 5


def factorialofNumber3 n
  return 1 if n == 0
  n = n * factorialofNumber3(n - 1)
end


puts factorialofNumber3 3
