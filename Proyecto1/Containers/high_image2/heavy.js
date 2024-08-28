//SCRIPT DE ALTO CONSUMO CON JAVASCRIPT
//PROGRAMA CON UN CICLO WHILE INFINITO

function heavyOperation() {
    async function run() {
        while (true) {
            // Realizar operaciones matemáticas intensivas
            const list = [];
            for (let j = 0; j < 10000; j++) {
                for (let i = 0; i < 100; i++) {
                    list.push(Math.pow(i, 5));  // Operación matemática intensiva
                }
            }

            console.log("Operación computacional de nivel alto corriendo...");

            // Pausa breve de 100 milisegundos
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    }

    run();
}

heavyOperation();

