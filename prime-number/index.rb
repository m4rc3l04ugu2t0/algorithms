# O(1): Tempo de execução constante. O algoritmo sempre leva o mesmo tempo, independentemente do tamanho da entrada.
# O(n): Tempo de execução linear. O tempo de execução aumenta diretamente proporcionalmente ao tamanho da entrada.
# O(n^2): Tempo de execução quadrático. O tempo de execução aumenta quadraticamente com o tamanho da entrada.
# O(log n): Tempo de execução logarítmico. O tempo de execução aumenta proporcionalmente ao logaritmo do tamanho da entrada.
# O(n!): Tempo de execução fatorial. O tempo de execução aumenta exponencialmente com o tamanho da entrada.

def is_prime n
  return false if n < 2

  (2..Math.sqrt(n)).each do |i|
    return false if n % i == 0
  end

  return true
end

puts is_prime 100

// O(sqrt(n))
