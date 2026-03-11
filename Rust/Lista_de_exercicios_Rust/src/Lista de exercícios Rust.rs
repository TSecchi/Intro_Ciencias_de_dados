use std::io;

// ========================================
// Exercício 1
// Imprimir a mensagem "Olá terráqueos!!"
// ========================================

fn exercicio1() {
    println!("Olá terráqueos!!");
}

// ========================================
// Exercício 2
// Ler um número e mostrar a mensagem
// ========================================

fn exercicio2() {

    let mut numero = String::new();

    println!("Digite um número:");

    io::stdin()
        .read_line(&mut numero)
        .expect("Erro ao ler");

    println!("O número informado foi {}", numero.trim());
}

// ========================================
// Exercício 3
// Ler dois números e mostrar a soma
// ========================================

fn exercicio3() {

    let mut n1 = String::new();
    let mut n2 = String::new();

    println!("Digite o primeiro número:");
    io::stdin().read_line(&mut n1).unwrap();

    println!("Digite o segundo número:");
    io::stdin().read_line(&mut n2).unwrap();

    let n1: f64 = n1.trim().parse().unwrap();
    let n2: f64 = n2.trim().parse().unwrap();

    println!("A soma é: {}", n1 + n2);
}

// ========================================
// Exercício 4
// Ler 4 notas e calcular a média
// ========================================

fn exercicio4() {

    let mut notas = Vec::new();

    for i in 1..=4 {

        let mut nota = String::new();

        println!("Digite a nota {}:", i);

        io::stdin().read_line(&mut nota).unwrap();

        let nota: f64 = nota.trim().parse().unwrap();

        notas.push(nota);

    }

    let media = (notas[0] + notas[1] + notas[2] + notas[3]) / 4.0;

    println!("A média é: {}", media);

}

// ========================================
// Exercício 5
// Converter metros para centímetros
// ========================================

fn exercicio5() {

    let mut metros = String::new();

    println!("Digite metros:");

    io::stdin().read_line(&mut metros).unwrap();

    let metros: f64 = metros.trim().parse().unwrap();

    println!("Centímetros: {}", metros * 100.0);

}

// ========================================
// Exercício 6
// Calcular área do círculo
// ========================================

fn exercicio6() {

    let mut raio = String::new();

    println!("Digite o raio:");

    io::stdin().read_line(&mut raio).unwrap();

    let raio: f64 = raio.trim().parse().unwrap();

    let area = std::f64::consts::PI * raio.powi(2);

    println!("Área: {}", area);

}

// ========================================
// Exercício 7
// Área do quadrado e dobro
// ========================================

fn exercicio7() {

    let mut lado = String::new();

    println!("Digite o lado:");

    io::stdin().read_line(&mut lado).unwrap();

    let lado: f64 = lado.trim().parse().unwrap();

    let area = lado.powi(2);

    println!("Área: {}", area);
    println!("Dobro da área: {}", area * 2.0);

}

// ========================================
// Exercício 8
// Calcular salário mensal
// ========================================

fn exercicio8() {

    let mut valor_hora = String::new();
    let mut horas = String::new();

    println!("Valor por hora:");
    io::stdin().read_line(&mut valor_hora).unwrap();

    println!("Horas trabalhadas:");
    io::stdin().read_line(&mut horas).unwrap();

    let valor_hora: f64 = valor_hora.trim().parse().unwrap();
    let horas: f64 = horas.trim().parse().unwrap();

    println!("Salário: R$ {}", valor_hora * horas);

}

// ========================================
// Exercício 9
// Fahrenheit para Celsius
// ========================================

fn exercicio9() {

    let mut f = String::new();

    println!("Temperatura em Fahrenheit:");

    io::stdin().read_line(&mut f).unwrap();

    let f: f64 = f.trim().parse().unwrap();

    let c = 5.0 * ((f - 32.0) / 9.0);

    println!("Celsius: {}", c);

}

// ========================================
// Exercício 10
// Celsius para Fahrenheit
// ========================================

fn exercicio10() {

    let mut c = String::new();

    println!("Temperatura em Celsius:");

    io::stdin().read_line(&mut c).unwrap();

    let c: f64 = c.trim().parse().unwrap();

    let f = (c * 9.0 / 5.0) + 32.0;

    println!("Fahrenheit: {}", f);

}

// ========================================
// Exercício 11
// Operações matemáticas
// ========================================

fn exercicio11() {

    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut n3 = String::new();

    println!("Primeiro número inteiro:");
    io::stdin().read_line(&mut n1).unwrap();

    println!("Segundo número inteiro:");
    io::stdin().read_line(&mut n2).unwrap();

    println!("Número real:");
    io::stdin().read_line(&mut n3).unwrap();

    let n1: i32 = n1.trim().parse().unwrap();
    let n2: i32 = n2.trim().parse().unwrap();
    let n3: f64 = n3.trim().parse().unwrap();

    let r1 = (2 * n1) as f64 * (n2 as f64 / 2.0);
    let r2 = (3 * n1) as f64 + n3;
    let r3 = n3.powi(3);

    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);

}

// ========================================
// Exercício 12
// Calcular peso ideal
// ========================================

fn exercicio12() {

    let mut altura = String::new();

    println!("Digite sua altura:");

    io::stdin().read_line(&mut altura).unwrap();

    let altura: f64 = altura.trim().parse().unwrap();

    let peso = (72.7 * altura) - 58.0;

    println!("Peso ideal: {} kg", peso);

}