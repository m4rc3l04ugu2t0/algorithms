def fibonacci n
  fib = [0, 1]
  count = 2

  while count < n
    fib[count] = fib[count -1] + fib[count -2]
    count += 1
  end

  fib
end

p fibonacci 7
