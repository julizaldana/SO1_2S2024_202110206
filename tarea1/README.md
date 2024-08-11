## <div align="center">Tarea #1 - Módulos de Kernel</div>
##### <div align="center">Sistemas Operativos 1 - Sección A</div>
##### <div align="center">10 de agosto de 2024</div>
___

En el archivo [tarea1_202110206.c](./tarea1_202110206.c); se define en lenguaje C, la mayoría de funciones y macros utiles para poder leer, trabajar información para módulos.

Funciones a considerar:

* La funcion **show_processes()**; iterará sobre la lista de procesos del sistema, imprimiendo todos los procesos padres y sus procesos hijos asociados.
* Se iteran sobre todos los procesos del sistema utilizando la macro **for_each_process**. Es un macro, que itera por la lista de todos los procesos de un sistema.
* Se utilizan el struct **task_struct** que contiene toda la informacion sobre un proceso.
* Con **list_for_each(list, &task->children)** itera sobre la lista de todos los procesos hijos; una lista que contiene todos los procesos hijos del proceso padre.

* Con la función **memandprocessesinfo_show()** se escribe la información de los procesos y la información de la memoria en el archivo que se creará en la carpeta.

* Siempre se utilizan los macros **__init** para dar permisos en la carpeta /proc y **__exit** para eliminar el archivo dentro de /proc.

______

**Comandos a utilizar**

Para la generación de archivos y compilación del archivo C.
```bash
make
```
Se instala/carga el módulo
```bash
sudo insmod tarea1_202110206.ko
```
Ver log de kernel, con mensaje de que se cargó el módulo
```bash
sudo dmesg | tail -n 20
```
Verificar si está activo el módulo
```bash
lsmod | grep tarea1_202110206
```
Ver archivo creado en /proc, y verificar su información.
```bash
cat /proc/memandprocessesinfo
```
___
**Comandos extras**

Para borrar automaticamente todos los archivos generados con el comando make.
```bash
make clean
```
Para desinstalar y eliminar el modulo.
```bash
sudo rmmod tarea1_202110206
```

___

**Salida**

En el archivo [memandprocessesinfo](./memandprocessesinfo.txt) se puede ver un ejemplo de la salida en /proc. Donde se muestra la información requerida, que sería la información básica de memoria y la información de procesos.