function fibonacci(n) {
    let first = 0;
    let second = 1;
    let nextNumber = 1;

    const sequence = [0, 1];

    while (sequence.length < n) {
        nextNumber = first + second;
        first = second;
        second = nextNumber;
        sequence.push(nextNumber);
    }

    return sequence;
}
console.log(fibonacci(20));

function fibonacci2(n) {
    let fib = [0, 1];
    for (let i = 2; i < n; i++) {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    return fib;
}

console.log(fibonacci2(20));

// A sequência de Fibonacci é uma série de números onde cada número é a soma dos dois números
//  anteriores. A sequência começa com 0 e 1, e os números subsequentes são gerados adicionando os doi
//  s números anteriores. A sequência começa assim:

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...

// Por exemplo, para obter o próximo número na sequência, você soma os dois números anteriores:

// 1 + 1 = 2
// 1 + 2 = 3
// 2 + 3 = 5
// 3 + 5 = 8
// e assim por diante.

// A sequência de Fibonacci é amplamente conhecida e tem várias aplicações em matemática,
//  ciência da computação e até mesmo em áreas como finanças e biologia. Ela aparece em muitos
//  contextos naturais, como a disposição de folhas em plantas, a estrutura de conchas de moluscos e
//   até mesmo na proporção áurea, que é encontrada em muitos aspectos da natureza e da arte. Além disso, a
//    sequência de Fibonacci é usada em algoritmos e problemas computacionais, como na geração de
//     números aleatórios, em algoritmos de busca e muito mais.
