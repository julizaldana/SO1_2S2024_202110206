//SCRIPT DE BAJO CONSUMO CON JAVASCRIPT
//SERVIDOR SENCILLO

const http = require('http');

const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end('Hola este es un servidor con JavaScript\n');
});

server.listen(port, hostname, () => {
    console.log(`Servidor corriendo en http://${hostname}:${port}/`);
});
