//SCRIPT DE ALTO CONSUMO CON JAVASCRIPT
//PROGRAMA CON UN CICLO WHILE INFINITO

function heavyOperation() {
    let num = 1;
    while (true) {
        num *= Math.random();
        console.log("Operación computacional de nivel alto corriendo...");
    }
}

heavyOperation();
