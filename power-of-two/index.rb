def isPowerOfTwo n
  return false if n < 1

  while n > 1
    return false if n % 2 != 0
    n = n/2
  end

  true
end

# puts isPowerOfTwo 4

def isPowerOfTwoBitWise n
  return false if n < 1

  (n & (n - 1)) == 0
end

puts isPowerOfTwoBitWise 5
