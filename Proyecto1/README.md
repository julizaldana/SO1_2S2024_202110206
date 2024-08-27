## <div align="center">Proyecto #1 </div>
##### <div align="center">Sistemas Operativos 1 - Sección A</div>
##### <div align="center">8 de septiembre de 2024</div>
___

#### **<div align="center">Introducción</div>**



___

#### **<div align="center">Creación de Contenedores con Script y Cronjob</div>**

<div align="center"><img src="./Images/dockerlogo.png" width="200"></div>


Para la creación de los contenedores con un script en Bash; se tomó en cuenta la restricción de los tipos de imagenes que se deben de crear en Docker. Que serían:

* **Imágenes de alto consumo**
    * [high_image1](./Containers/high_image1/main.py): Se centra en un programa en Python con un ciclo infinito y una pausa de 0.1 segundos, que muestra un mensaje continuo en cada iteración con la fecha y hora del día actual.
    * [high_image2](./Containers/high_image2/heavy.js): Se centra en un programa en JavaScript que contiene un ciclo while infinito que realiza una operación matemática aleatoria. En cada iteración, una variable se multiplica por un número generado aleatoriamente y se reasigna continuamente. 
* **Imágenes de bajo consumo**
    * [low_image1](./Containers/low_image1/app.js) Se centra en un servidor sencillo en JavaScript.
    * [low_image2](./Containers/low_image2/app.py): Se centra en un servidor en Python, utilizando Flask.

