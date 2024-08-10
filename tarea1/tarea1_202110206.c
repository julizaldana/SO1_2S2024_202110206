#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>  //Brinda funciones para crear archivos en /proc
#include <linux/seq_file.h> //Brinda funciones para escribir en archivos
#include <linux/mm.h>   //Brinda funciones para manejar la memoria
#include <linux/sched.h> //Brinda funciones para manejar y leer procesos
#include <linux/timer.h> 
#include <linux/jiffies.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Julio Zaldaña");
MODULE_DESCRIPTION("Modulo para leer información de Memoria, Procesos Padres y Procesos Hijos");
MODULE_VERSION("1.0");

//NOMBRE: JULIO ALEJANDRO ZALDAÑA RÍOS
//CARNET: 202110206
//SISTEMAS OPERATIVOS 1 - SECCION A
//TAREA 1
//-------------------------------------------------------------------

#define PROC_NAME "memandprocessesinfo" //nombre de archivo que se creará en /proc

/*  
    La funcion show_processes(); iterará sobre la lista de procesos del sistema, imprimiendo toda su información;
    sobre procesos padres y sus hijos.
*/
static void show_processes(struct seq_file *m) {
    struct task_struct *task; // struct que contiene toda la informacion sobre un proceso.
    struct list_head *list;   // Puntero que sirve para iterar sobre la lista de procesos hijos.

    // Se iteran sobre todos los procesos del sistema utilizando la macro for_each_process.
    // - for_each_process es un macro, que itera por la lista de todos los procesos de un sistema
    for_each_process(task) {
        //Se imprime el nombre y el PID del proceso padre en el archivo /proc
        seq_printf(m, "Proceso Padre: %s (PID: %d)\n", task->comm, task->pid);

        // Se iteran sobre la lista de todos los procesos hijos; una lista que contiene todos los procesos hijos del proceso padre
        list_for_each(list, &task->children) {
            struct task_struct *child;
            //Se obtiene el struct del proceso hijo a partir de la lista de hijos.
            //sibling conecta al proceso hijo con la lista de hermanos (otros hijos del mismo padre)
            child = list_entry(list, struct task_struct, sibling);
            //Se imprime el nombre y el PID del proceso hijo en el archivo /proc
            seq_printf(m, "  - Proceso Hijo: %s (PID: %d)\n", child->comm, child->pid);
        }
    }
}


static int memandprocessesinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si;

    si_meminfo(&si);

    // Se imprime información de memoria
    seq_printf(m, "\n--- Información de Memoria ---\n");
    seq_printf(m, "Total RAM: %lu KB\n", si.totalram * 4);
    seq_printf(m, "Free RAM: %lu KB\n", si.freeram * 4);

    // Se imprimen los procesos padre e hijos, llamando a la función de show_processes()
    seq_printf(m, "\n--- Procesos Padre e Hijos ---\n");
    show_processes(m);

    return 0;
}

/* 
    Esta función se ejecuta cuando se abre el archivo en /proc
    - single_open: se encarga de abrir el archivo y ejecutar la función sysinfo_show
*/
static int memandprocessesinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, memandprocessesinfo_show, NULL);
}

/* 
    Esta estructura contiene las operaciones a realizar cuando se accede al archivo en /proc
    - proc_open: se ejecuta cuando se abre el archivo
    - proc_read: se ejecuta cuando se lee el archivo
*/
static const struct proc_ops memandprocessesinfo_ops = {
    .proc_open = memandprocessesinfo_open,
    .proc_read = seq_read,
};


/*
Macro para crear y dar permisos dentro /proc 
*/

static int __init memandprocessesinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &memandprocessesinfo_ops);
    printk(KERN_INFO "memandprocessesinfo module loaded\n");
    return 0;
}

/*
Macro para eliminar dentro de /proc
*/

static void __exit memandprocessesinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "memandprocessesinfo module unloaded\n");
}

module_init(memandprocessesinfo_init);
module_exit(memandprocessesinfo_exit);
