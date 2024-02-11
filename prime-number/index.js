function isPrime(n) {
    if ((n % 2 === 0 && n > 2) || n < 2) return false;
    let result = true;
    for (let i = 1; i < n - 1; i++) {
        if (n % (n - i) === 0) {
            result = false;
            break;
        }
    }

    return result;
}
// console.log(isPrime(1));
// console.log(isPrime(5));
// console.log(isPrime(4));

function isPrime2(n) {
    if (n < 2) {
        return false;
    }

    for (let i = 2; i <= Math.sqrt(n); i++) {
        if (n % i === 0) {
            return false;
        }
    }

    return true;
}

console.log(isPrime2(1));
console.log(isPrime2(5));
console.log(isPrime2(4));

// O(sqrt(n))
