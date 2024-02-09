function factorialofNumber(n) {
    let factorial = 1;

    for (let i = 2; i <= n; i++) {
        factorial = i * factorial;
    }
    return factorial;
}
console.log(factorialofNumber(4));
// Big - O = O(n)
